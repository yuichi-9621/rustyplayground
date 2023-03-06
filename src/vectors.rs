pub fn run(){
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    
    println!("{:?}", vector);

    vector.push(5);
    vector.push(25);

    println!("{:?}", vector);

    //pop last value
    vector.pop();

    println!("{:?}", vector);

    for x in vector.iter(){
        println!("Numbers: {}", x); //don't need :? since not printing array
    }

    for x in vector.iter_mut(){
        // *x is a copy of each value in vector which is mutable
        *x = *x * 2; //could also be written as *x *= 2
        println!("{}", x);
    }
    println!("{:?}", vector);

}