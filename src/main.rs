use clap::{Parser, Subcommand};
use ssh_key::private::{Ed25519Keypair, Ed25519PrivateKey};
use ssh_key::public::Ed25519PublicKey;
use ssh_key::{LineEnding, PrivateKey};
use std::{fs, process::exit};

pub mod decoder;
pub mod encoder;
pub mod util;

#[derive(Parser, Debug)]
#[clap(disable_version_flag = true)] // disable the -V, --version flag
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Backup {
        #[arg(long, default_value = "")]
        file: String,
    },
    Restore {
        #[arg(long, default_value = "")]
        seed: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Command::Backup { file } => {
            if file == "" {
                println!("You must provide the private SSH key path using --file argument!");
                exit(1);
            }

            let private_key_data =
                fs::read_to_string(file).expect("Failed to read private key file");
            let private_key =
                PrivateKey::from_openssh(&private_key_data).expect("Failed to decode private key");

            let mnemonic = encoder::encode::encode(private_key).unwrap();
            println!("{}", mnemonic);
        }
        Command::Restore { seed } => {
            if seed == "" {
                println!("You must provide seed to restore from using --seed argument!");
                exit(1);
            }
            let private_key_bytes = decoder::decode::decode(seed);
            let key = ed25519_dalek::SigningKey::from_bytes(&private_key_bytes);

            let ed25519pub = Ed25519PublicKey(key.verifying_key().to_bytes());
            let ed25519priv = Ed25519PrivateKey::from_bytes(&key.to_bytes());

            let keypair = Ed25519Keypair {
                public: ed25519pub,
                private: ed25519priv,
            };

            let sshkey = ssh_key::PrivateKey::from(keypair);
            let ssh_key_string = sshkey.to_openssh(LineEnding::LF).unwrap();

            print!("{}", *ssh_key_string);
        }
    };
}
