mod utils;

pub fn decode(mnemonic: String) -> [u8; 32] {
    *utils::mnemonic_to_entropy(&mnemonic)
}
