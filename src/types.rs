//primitive types
//Int: u8, i8, u16, i16, ..., u128, i128 bits of memor y
//Floats: f32, f64
//Boolean (bool)
//Characters (char)
//Tuples
//Arrays

pub fn run(){
    let x = 1; //Default => i32

    let y = 2.3; //Default => f64

    let z: i64 = 4545454545454545;

    println!("{} is i32, and {} is f64",
    x, y
);
    println!{"Max bits of i32 is {}", std::i32::MAX}
    println!{"Max bits of i64 is {}", std::i64::MAX}

    //Boolean
    let is_active: bool = true;

    //Boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let smile = "\u{1f600}";

    println!("{:?}", (x, y, z, is_active, is_greater, a1, smile));

}