use std::collections::{HashSet};

pub fn check(candidate: &str) -> bool {
    let mut m: HashSet<char> = HashSet::new();
    candidate
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| m.insert(c))
}

