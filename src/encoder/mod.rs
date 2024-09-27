use bip39::Mnemonic;
use ed25519_dalek::SigningKey;
use ssh_key::PrivateKey;

pub fn encode(private: PrivateKey) -> Result<String, Box<dyn std::error::Error>> {
    let secret = SigningKey::from_bytes(private.key_data().ed25519().unwrap().private.as_ref());
    let entropy = secret.as_bytes();

    let mnemonic = Mnemonic::from_entropy(entropy)?;
    Ok(mnemonic.to_string())
}
