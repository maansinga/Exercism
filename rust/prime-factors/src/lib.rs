fn gather_primes_under(n: u64) ->Vec<u64>{
    let mut num = n;

    let mut primes: Vec<u64> = Vec::new();
    primes.push(2);

    let mut prime_factors: Vec<u64> = Vec::new();

    while(num > 1){
        let last_prime = primes.last().unwrap();
        if *last_prime > num{
            break;
        }
        if num % last_prime == 0{
            prime_factors.push(last_prime.clone());
            num = num / last_prime;
        }else{
            //find next prime
            let next_prime = (*last_prime+1..)
                .filter(|np| {
                    primes
                        .iter()
                        .all(|p| np % p != 0)
                })
                .next()
                .unwrap();

            primes.push(next_prime.clone());
            if next_prime > num/2{
                prime_factors.push(num.clone());
                break;
            }
        }
    }


    prime_factors
}

pub fn factors(n: u64) -> Vec<u64> {
//    let primes = primes_under(n);
//    let mut n_factors: Vec<u64> = Vec::new();
//    let mut ntest = n;
//
//    // unless the remainder of all factorizations is 0
//    //     if the top prime number is less than current number
//    //      search for primes halfway up to the number
//    //
//    //     for all the prime numbers in primes
//    //      check if anyone is a prime factor
//    //      if anyone is prime factor
//    //          divide the number by that prime number and repeat
//
//
//
//    for k in primes{
//        while ntest % k == 0{
//            n_factors.push(k.clone());
//            ntest = ntest / k;
//        }
//    }
//
//    n_factors
    gather_primes_under(n)
}
