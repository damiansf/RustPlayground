pub fn run() {
    println!("Test print from file");

    println!("{} {}", "Hello", "World!");

    println!("{1} {0}", "Hello", "World!");

    println!("{name} does stuff", name = "Damian");

    println!("Binary: {:b}, Hex {:x}, Octal {:o}", 10, 10, 10);

    println!("{:?}", (12, true, "hello"));
}