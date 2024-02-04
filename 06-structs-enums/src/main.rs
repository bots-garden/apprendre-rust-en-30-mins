struct Person {
    name: String,
    age: u32,
}

enum Status {
    Success,
    Failure(String),
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    match person.age {
        0..=17 => println!("Minor"),
        18..=64 => println!("Adult"),
        _ => println!("Senior"),
    }

    let result = Status::Success;
    match result {
        Status::Success => println!("Operation succeeded"),
        Status::Failure(msg) => println!("Operation failed with message: {}", msg),
    }
}