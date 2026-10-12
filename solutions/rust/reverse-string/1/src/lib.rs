pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    // .chars() iterates on a string
    // .rev(): reverse string direction
    // .collect():  takes ownership of an iterator and produces whichever collection type you request
    input.chars().rev().collect()
}
