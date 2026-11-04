pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    // find length on n, return the number at the last index
    // n only divisible by 1 and itself
    let mut position = 0;
    let mut candidate = 2;

    loop {
        let mut is_prime = true;
        let mut divisor = 2;
        while divisor <= candidate / divisor {
            if candidate % divisor == 0 {
                is_prime = false;
                break;
            }
            divisor += 1;
        }

        if is_prime {
            if position == n {
                return candidate;
            }
            position += 1;
        }
        candidate += 1;
    }
}
