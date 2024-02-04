fn main() {
    // String
    let mut greeting = String::from("Hello, ");
    greeting.push_str("world!");
    println!("{}", greeting);

    // Vec
    let numbers = vec![1, 2, 3, 4, 5];
    for num in &numbers {
        println!("{}", num);
    }
}