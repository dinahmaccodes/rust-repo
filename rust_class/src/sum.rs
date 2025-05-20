

pub fn calculate_sum(x: u8, y: u8) -> u8 {
    const U8MAX: u8 = u8::MAX;
    println!("The max value of u8 is: {}", U8MAX);
    assert!(x <= U8MAX, "x is too large");
    assert!(y <= U8MAX, "y is too large");
    let result: u8 = x + y;
    assert!(result <= U8MAX, "sum is too large");
    return result;
}