fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // ownership transférée à s2
    // println!("{}", s1); // erreur car s1 ne possède plus la chaîne.
    println!("{}", s2);

    let s3 = String::from("Hello");
    let s4 = &s3; // borrowing de l'ownership
    println!("{}", s3); // Cela est autorisé grâce au borrowing.
    println!("{}", s4);
}