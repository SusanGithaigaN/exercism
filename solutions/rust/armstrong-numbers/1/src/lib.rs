pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    // check num len
    // let mut num_len = num as u32;
    // let num_len = num.ilog10() + 1;
    // bool to str
    let num_str = num.to_string();
    // println!("{}", num_len.chars().count());
    let num_len = num_str.chars().count() as u32;

    let total_sum: u32 = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_len))
        .sum();

    // check if the sum == num
    total_sum == num
}
