pub fn run() {
    let person: (&str, &str, i8) = ("Damian", "Van", 24);

    println!("{}, {}, {}", person.0, person.1, person.2);
}