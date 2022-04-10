pub fn run() {
    let y = 1.5;
    let x = 1;

    let z: i64 = 45454545;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let value: bool = true;

    let test_char = 'a';
    let face = '\u{1F600}';

    print!("{:?}", (x, y, z, value, 10<9, test_char, face));
}