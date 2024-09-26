use crate::util::get_wordlist;

pub fn mnemonic_to_entropy(mnemonic: &str) -> &[u8; 32] {
    let wordlist = get_wordlist::get_wordlist();

    let words: Vec<&str> = mnemonic.split_whitespace().collect();

    if words.len() != 24 {
        panic!("This function only supports 24-word mnemonics for 256-bit entropy");
    }

    let mut bits = String::new();
    for word in &words {
        let index = wordlist.get(*word).expect("Word not found in wordlist");
        bits.push_str(&format!("{:011b}", index));
    }

    let entropy_bits = &bits[..bits.len() - 8];

    let mut entropy_bytes = [0u8; 32];
    for (i, byte) in entropy_bytes.iter_mut().enumerate() {
        let byte_str = &entropy_bits[i * 8..(i + 1) * 8];
        *byte = u8::from_str_radix(byte_str, 2).unwrap();
    }

    Box::leak(Box::new(entropy_bytes))
}
