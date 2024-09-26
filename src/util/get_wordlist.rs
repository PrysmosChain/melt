use bip39::Language;
use std::collections::HashMap;

pub fn get_wordlist() -> HashMap<&'static str, usize> {
    let wordlist = Language::English.word_list();
    wordlist
        .iter()
        .enumerate()
        .map(|(i, &word)| (word, i))
        .collect()
}
