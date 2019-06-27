pub fn build_proverb(list: &[&str]) -> String {
    if list.len() > 0{
        list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.\n", window[0], window[1]))
            .fold("".to_string(), |acc, c| acc + &c) +
        &format!("And all for the want of a {}.", list[0])
    }else{
        String::new()
    }
}
