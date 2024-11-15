fn main() {
    data_types_example();
    integer_types_example();
}

fn data_types_example() {
    // String type
    let company_string = "My Company"; // string

    // Float type
    let rating_float = 4.5; // f64 by default

    // Integer type
    let employee_int = 100; // i32 by default

    // Type of boolean
    let is_active_bool = true; // bool

    // Char type
    let first_char = 'A'; // char_1
    let second_char = 'B'; // char_2
    let concatenated_char = format!("{}, {}", first_char, second_char); // string_concatenated
    let icon_char = '🚀'; // char_icon

    // Print all the variables
    println!("Company: {}", company_string);
    println!("Rating: {}", rating_float);
    println!("Employee: {}", employee_int);
    println!("Active: {}", is_active_bool);
    println!("First char: {}", concatenated_char);
    println!("Icon char: {}", icon_char);
    println!("---------------------------------");
}

fn integer_types_example() {
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