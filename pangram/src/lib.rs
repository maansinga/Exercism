use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let punch_card: HashSet<char> = HashSet::from(
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .clone()
            .collect()
    );

    let sentence_set: HashSet<char> = HashSet::from(
        sentence
            .to_ascii_lowercase()
            .chars()
            .clone()
            .collect()
    );

    punch_card.is_subset(&sentence_set)
}
