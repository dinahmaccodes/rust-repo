pub fn calc_sum_result(x: u8, y: u8) -> String {
    let result: u8 = x+y;
    if result%2 == 0 {
        "even".to_string()
    } else {
        "odd".to_string()
    }
        
}