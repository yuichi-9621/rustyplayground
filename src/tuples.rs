//Tuples
//Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Yuichi", "California", 25);
    println!("{} is from {}, and is {}years of age", person.0, person.1, person.2);
}