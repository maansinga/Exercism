pub fn raindrops(n: u32) -> String {
    let factors = [3, 5, 7];
    let mut collected_factors: Vec<i32> = Vec::new();

    for fac in factors.iter() {
        if n % fac == 0 {
            collected_factors.push(fac.clone() as i32);
        }
    }

    if collected_factors.is_empty() {
        format!("{}", n)
    } else {
        collected_factors
            .iter()
            .map(|x| match x {
                3 => "Pling",
                5 => "Plang",
                7 => "Plong",
                _ => "",
            })
            .fold("".to_owned(), |acc, s| format!("{}{}", acc, s))
    }
}
