// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //use debug traits to show array
    println!("{:?}", numbers);
    println!("Array length: {}", numbers.len());

    //get the first value
    println!("First num is: {}", numbers[0]);

    //replace numbers
    numbers[0] = 25;
    println!("First num is: {} now", numbers[0]);
    println!{"{:?}", numbers};

    //Arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); //printing out array so :?

    //println!("Slice the first 3: {}" )
}