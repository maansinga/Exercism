fn is_prime(x: &u32, primes: &mut Vec<u32>) -> bool {
    if primes
        .iter()
        .map(|p| x % *p == 0)
        .fold(false,|accum, cur| accum || cur)
    {
        false
    }else{
        primes.push(x.clone());
        true
    }
}

pub fn nth(n: u32) -> u32 {
    if n == 0 as u32 {
        2
    }else{
        let mut primes: Vec<u32> = Vec::new();
        primes.push(2);

        (3..)
            .filter(move |x| is_prime(x, &mut primes))
            .nth((n-1) as usize)
            .unwrap()
    }
}