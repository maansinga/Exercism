use std::collections::HashSet;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let word = word.to_ascii_uppercase();
    let hashset_1: HashSet<char> =
        "AEIOULNRST".chars().clone().collect();
    let hashset_2: HashSet<char> =
        "DG".chars().clone().collect();
    let hashset_3: HashSet<char> =
        "BCMP".chars().clone().collect();
    let hashset_4: HashSet<char> =
        "FHVWY".chars().clone().collect();
    let hashset_5: HashSet<char> =
        "K".chars().clone().collect();
    let hashset_8: HashSet<char> =
        "JX".chars().clone().collect();
    let hashset_10: HashSet<char> =
        "QZ".chars().clone().collect();

    word
        .chars()
        .map(|x| {
            match x {
                k if hashset_1.contains(&k) => 1,
                k if hashset_2.contains(&k) => 2,
                k if hashset_3.contains(&k) => 3,
                k if hashset_4.contains(&k) => 4,
                k if hashset_5.contains(&k) => 5,
                k if hashset_8.contains(&k) => 8,
                k if hashset_10.contains(&k) => 10,
                _ => 0
            }
        })
        .fold(0, |a, c| a + c)
}
