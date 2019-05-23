fn is_prime(x: &u64, primes: &mut Vec<u64>) -> bool {
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

pub fn factors(n: u64) -> Vec<u64> {
    let mut divn = n;
    let mut primes: Vec<u64> = Vec::new();
    primes.push(2u64);

    let mut primes_iter = (1..).filter(|x| is_prime(x, &mut primes));

    let mut dispatch_vec: Vec<u64> = Vec::new();
    let mut prime = primes_iter.next().unwrap();
    while divn > 1 {
        if divn % prime == 0{
            dispatch_vec.push(prime.clone());
            divn = divn/prime;
        }else{
            prime = primes_iter.next().unwrap();
        }
    }

    dispatch_vec
}
