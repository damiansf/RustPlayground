use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[2] = 900;

    println!("{:?}", numbers);

    println!("{}", numbers[0]);

    println!("{}", numbers.len());

    println!("{}", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];

    println!("{:?}", slice);

    numbers.push(90);

    println!("{:?}", numbers);

    numbers.pop();

    println!("{:?}", numbers);

    for x in numbers.iter() {
        println!("{}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
        println!("{}", x);
    }

    println!("{:?}", numbers);
}