pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    // item at index i +1
    // use left shift
    if s<1 || s> 64{
        panic!{"square must be within the chessboard"}
    }
    1u64 << (s-1)
}

pub fn total() -> u64 {
    // todo!();
    u64::MAX
}
