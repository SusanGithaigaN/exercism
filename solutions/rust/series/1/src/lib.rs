pub fn series(digits: &str, len: usize) -> Vec<String> {
    // todo!("What are the series of length {len} in string {digits:?}")
    // guard
    if len>digits.len(){
        return Vec::new();
    }

    let mut result = Vec::new();
       // A string of length n has (n - len + 1) windows of size len.
    for start in 0..=(digits.len() - len) {
        let slice = &digits[start..start + len]; 
        result.push(slice.to_string());  
    }
    result
}
