/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut place = (1..=10).rev().into_iter();
    let mut accum = 0;
    let mut sym_count = 0;
    for c in isbn.chars(){
        match c{
            k @ '0'..='9' => {
                accum = accum + k.to_digit(10).unwrap() * place.next().unwrap();
                sym_count += 1;
            },
            'X'|'x' => {
                if sym_count != 10 { return false; }
                else{
                    accum = accum + 10 * place.next().unwrap();
                    sym_count += 1;
                }
            },
            '-'=> continue,
            _ => { return false; }
        }
    }
    false
}
