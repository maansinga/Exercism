pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    match upper_bound {
        1 => vec![],
        2 => vec![2],
        k if k > 2 => {
            let mut primes = vec![2];
            let mut i = 3;

            while *primes.last().unwrap() < upper_bound {
                if i > upper_bound {
                    break;
                } else if primes.iter().all(|x| i % *x != 0) {
                    primes.push(i)
                }
                i += 2;
            }

            primes
        }
        _ => vec![]
    }
}
