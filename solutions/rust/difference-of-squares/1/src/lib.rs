pub fn square_of_sum(n: u32) -> u32 {
    // todo!("square of sum of 1...{n}")
    let mut sum = 0;

    for i in 1..=n{
        sum+=i;
    }
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")
    let mut sum_squares = 0;

    for i in 1..=n{
        // sum of squares on all values in n
        // square all calues first
        let square = i *i;
        sum_squares += square;
    }
    sum_squares
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
