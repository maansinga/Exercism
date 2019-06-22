/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let all_valid_chars_flag = isbn
        .chars()
        .all(|x| {
            match x {
                '0'..='9' | '-' | 'x' | 'X' => true,
                _ => false
            }
        });

    if !all_valid_chars_flag { return false; }

    let has_x_in_right_place = if isbn.chars().any(|x| x == 'X') {
        isbn.chars().last().unwrap() == 'X'
    } else { true };

    if !has_x_in_right_place { return false; }

    let filtered = isbn
        .chars()
        .filter(|x| {
            match x {
                '0'..='9' | 'X' | 'x' => true,
                _ => false
            }
        });

    let post_filter_count_10 = filtered.clone().collect::<String>().len() == 10;

    if !post_filter_count_10 { return false; }

    filtered
        .map(|x| {
            match x {
                '0'..='9' => x.to_digit(10).unwrap(),
                'x' | 'X' => 10,
                _ => 0
            }
        })
        .zip((1..=10).rev())
        .fold(0, |a, (x, y)| {
            a + x * y
        }) % 11 == 0
}
