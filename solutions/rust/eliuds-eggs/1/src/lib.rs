pub fn egg_count(display_value: u32) -> usize {
    // todo!("count the eggs in {display_value}")
    // 1 for existing egg,, 0 for an empty spot
    // convert number from binary to decimal
    // return result
    let mut total_eggs = 0;
    let mut tray = display_value;

    while tray > 0{
        if tray % 2 == 1{
            total_eggs +=1;
        }
        tray /= 2;
    }
    total_eggs
}
