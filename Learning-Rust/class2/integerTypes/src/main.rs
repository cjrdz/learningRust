fn main() {
    // Integer types
    let result = 10; // i32 by default

    // unsigned integer types
    let age: u32 = 20; // 32-bit unsigned integer

    let ageresult = age - result as u32; // subtraction of unsigned integer from signed integer

    // signed integer types
    let sum: i32 = 5 + 20; // 32-bit signed integer

    // isize and usize types
    let mark: isize = 10; // isize is architecture dependent

    // usize is architecture dependent
    let count: usize = 30; // usize is architecture dependent

    // Print all the variables
    println!("result value is = {}", result);
    println!("sum is = {}", sum);
    println!("age is = {}", age);
    println!("age subtraction {} - result {} is = {}", age, result, ageresult);
    println!("mark is = {} and count is = {}", mark, count);
}
