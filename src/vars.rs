//variables hold primitive to data
//variables are immutable by default
//Rust is block-scoped language

pub fn run(){
    let name = "Yuichi";
    let mut age = 25;
    println!("I'm {} and I'm {} now", name, age);
    age = 26;
    println!("I'm turning {}", age);

    //constant is usually uppercase
    //need to define type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple variables
    let (my_Name, born) = ("Yuichi", 1997);
    println!("My name is {}, and I was born in {}", my_Name, born);
}