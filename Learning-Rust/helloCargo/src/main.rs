// My first Rust program
fn main() {
    // Print 1 line
    println!("Hello, world! from Cargo");

    // Print with arguments with position
    println!("My name is {0} {1}", "Carlos", "Rodriguez");

    // Print with int argument
    println!("I am {} years-old", 25);

    // Print multiple named arguments
    println!("{myself} {goal} {object}",
        myself = "I am a software engineer",
        goal = "and I am learning",
        object = "Rust programming language");

    // Print with number with extra zeros
    println!("I love you! {number:0<7}", number = 3);

    // Print with number with extra named arguments
    println!("I live driving up to 4{number:0>width$} KM/h", number = 0, width = 2);

    // Print with arguments with repetition position
    println!("My name is {0}, {1} {0}", "Bond", "James");
}