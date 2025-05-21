
mod sum;
mod sub;
mod div;
mod mult; 
mod even_sum;
mod sum_result;
mod fp;

fn main() {
    println!("Hello, big world!");
    // Basic Addition
    let sum: u8 = sum::calculate_sum(70, 9);
    println!("Sum of the numbers is {}", sum);
    // Subtraction
    let sub: u8 = sub::subtraction(20, 8);
    println!("Subtraction of the number is {}", sub);
    // Basic multiplication
    let mult: u16 = mult::multiplication(20,15);
    println!("The product of x and y is: {}", mult);
    // Basic division 
    let div: u8 = div::division(45, 15);
    println!("The result dividing x and y is: {}", div);

    // Basic summation for even result status
    let even_sum: bool = even_sum::even_sum_result(24, 9);
    println!("The sum is even: {}", even_sum);

    // Basic summation for result status
    let sum_result: String = sum_result::calc_sum_result(52, 8);
    println!("The sum is: {}", sum_result);
    //Basic floating point numbers
    let floating_point: f64 = fp::floating(2.5, 8.91);
    println!("points are: {}", floating_point);

}