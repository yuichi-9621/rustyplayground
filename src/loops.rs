pub fn run(){
    let mut count: i8 = 1;

    //infinite loop
    /*    
    loop {
        count += 1;
        println!("Number: {}", count);
        if count == 20 {
            break;
        }    
    } */


    //while loop (fizzbuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("Number: {}", count);
        }
        count += 1;
    }

    println!("");

    //For Range
    for x in 1..100 {
        if x % 6 == 0 {
            println!("Divisible by 6");
        } else if x % 2 == 0 {
            println!("Even number");
        } else if x % 3 == 0 {
            println!("Odd number");
        }
    }
}