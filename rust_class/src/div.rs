// Have for division 
pub fn division(x: u8, y: u8) -> u8 {
    let result: u8 = x/y;
    assert!(x > y, "First number is too small");
    assert!(x != 0, "Numerator cannot be zero");
    assert!(y != 0, "Denomenator cannot be zero");
    return result;
}