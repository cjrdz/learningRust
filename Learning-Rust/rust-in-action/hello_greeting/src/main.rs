fn greet_hello() {
    println!("Hello, world!");
    let spanish = "Hola mundo!";
    let germany  = "Grüß Gott!";
    let japan = "こんにちは";
    let regions = [spanish, germany, japan];
    for region in regions.iter(){
        print!("{} ", &region);
    }
}

fn main() {
    greet_hello();
}