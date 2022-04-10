struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Color_Tuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    }; 

    println!("{}, {}, {}", c.red, c.green, c.blue);

    c.red = 200;

    println!("{}, {}, {}", c.red, c.green, c.blue);

    let mut c2 = Color_Tuple(255, 0, 0);

    println!("{}, {}, {}", c2.0, c2.1, c2.2);

    c2.0 = 200;

    println!("{}, {}, {}", c2.0, c2.1, c2.2);

    let mut pers = Person::new("Damian", "SF");

    println!("{}, {}", pers.first_name, pers.last_name);

    println!("{}", pers.full_name());

    pers.set_last_name("test");

    println!("{}", pers.full_name());

    println!("{:?}", pers.to_tuple());

}