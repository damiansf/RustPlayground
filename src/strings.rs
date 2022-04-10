pub fn run() {
    let hello = "Hello";
    let mut test = String::from("Hello");

    println!("{}", hello);
    println!("{}", hello.len());

    println!("{}", test);
    println!("{}", test.len());

    test.push('W');

    println!("{}", test);
    println!("{}", test.len());

    test.push_str(" W");

    println!("{}", test);
    println!("{}", test.len());

    println!("{}", test.capacity());

    println!("{}", test.is_empty());

    println!("{}", test.contains("W"));

    println!("{}", test.replace("W", "Z"));

    for word in test.split_whitespace() {
        println!("{}", word);
    }

    let mut capped_string  = String::with_capacity(10);

    capped_string.push('a');

    capped_string.push('b');

    println!("{}", capped_string);

    assert_eq!(2, capped_string.len());

    assert_eq!(10, capped_string.capacity());
}