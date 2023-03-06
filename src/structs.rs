//Custom data types

//Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//Tuple Struct
struct Colorsss(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set new name
    fn set_newname(&mut self, first: &str, last: &str){
        self.last_name = last.to_string();
        self.first_name = first.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

//rgb is less than 255
pub fn run(){
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {}, {}, {}", c.red, c.green, c.blue);

    let mut colors = Colorsss(255, 255, 255);
    println!("Colorsss {}, {}, {}", colors.0, colors.1, colors.2);

    colors.0 = 100;

    println!("New Colorsss: {}, {}, {}", colors.0, colors.1, colors.2);

    //Person
    let mut p = Person::new("Yuichi", "Okuhama");
    println!("My name is {} {}", p.first_name, p.last_name);

    println!("{}", p.full_name());

    p.set_newname("Hugh", "Jackman");

    println!("{}", p.full_name());

    println!("Person Tuple {:?}", p.to_tuple());

}