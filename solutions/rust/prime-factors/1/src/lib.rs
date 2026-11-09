pub fn factors(n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")
    let mut n = n;
    // store prime factores
    let mut prime_factors = Vec::new();
    let mut smallest = 2;

        while smallest <= n / smallest {
        while n % smallest == 0 {
            prime_factors.push(smallest);
            n /= smallest;
        }
        smallest+= 1;
    }

    if n > 1 {
        prime_factors.push(n);
    }
    prime_factors
}
