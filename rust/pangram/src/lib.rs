use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut punch_card: HashSet<char> = HashSet::from(
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .clone()
            .collect()
    );

    for x in sentence.to_ascii_lowercase().chars() {
        punch_card.remove(&x);
    }

    punch_card.is_empty()
}
