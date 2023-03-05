// Arrays - Fixed list where elements are the same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //use debug traits to show array
    println!("{:?}", numbers);

    //get the first value
    println!("First num is: {}", numbers[0]);

    //replace numbers
    numbers[0] = 25;
    println!("First num is: {} now", numbers[0]);
    println!{"{:?}", numbers};
}