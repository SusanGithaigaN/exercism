pub fn is_leap_year(year: u64) -> bool {
    // todo!("true if {year} is a leap year")
    // conditions: year % 4=0 %100 ==0

    if year % 4 == 0 && (year %100 != 0 || year %400 ==0){
        true
    }else{
        false
    }
}
