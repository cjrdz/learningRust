fn main() {
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
}
