pub fn run () {
    let age: u8 = 19;
    let check_id: bool = false;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("What do you want to drink?");
    } else if age < 21 && check_id {
        println!("Go away");
    } else {
        println!("Let's see the id");
    }

    let is_of_age = if age >= 21 {true} else {false};

    println!("{}", is_of_age);
}