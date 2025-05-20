pub fn subtraction(x: u8, y:u8) -> u8 {
    assert!(x > y, "Provide larger number first");
    let result: u8 = x - y;
    return result;
}