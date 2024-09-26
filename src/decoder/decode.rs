use crate::util;

pub fn decode(mnemonic: String) -> [u8; 32] {
    *util::mnemonic_to_entropy(&mnemonic)
}
