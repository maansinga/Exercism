use core::str;
pub fn abbreviate(phrase: &str) -> String {
    if phrase.len() == 0 { "".to_owned() } else {
        phrase
            .split(Regex::new())
            .map(|word| {
                word
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_ascii_uppercase()
            }).collect()
    }
}
