pub fn run() {
    let name = "Damian";
    let mut age = 24;

    age = 25;

    println!("{}, {}", name, age);

    const ID: i32 = 001;

    println!("{}", ID);

    let (my_name, my_age) = ("Damian", 24);

    println!("{}, {}", my_name, my_age);
}