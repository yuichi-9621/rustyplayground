//Primitive str = Immutable fix-length string somewhere in memory
//String = Growable, heap-allocated data structure
//Use when need to modify the string data

pub fn run(){
    let mut konnichiwa = String::from("Konnichiwa ");

    /*
    strings could only be modified when:
     - Mutable
     - using String::
    */

    //push char
    konnichiwa.push('D');
    //push string
    konnichiwa.push_str("esu");
    
    //Capacity in bytes 
    println!("Capacity: {}", konnichiwa.capacity());

    //Check if empty
    println!("Is it empty? {}", konnichiwa.is_empty());

    //Contains substring? Uppercase, lowercase matters
    println!("Does it contain 'Desu'? {}", konnichiwa.contains("Desu"));
    println!("Does it contain 'desu'? {}", konnichiwa.contains("desu"));
    
    //Replace
    println!("Replace Konnichiwa with 'Konbanwa': {}", 
    konnichiwa.replace("Konnichiwa","Konbanwa")
);

    //Split by whitespace
    for blob in konnichiwa.split_whitespace(){
        println!("{}", blob);
    }

    println!("Konnichiwa length: {}", konnichiwa.len());
    println!("{}", konnichiwa);

    //Create string with capacity
    let mut s = String::with_capacity(10);
    
    s.push('a');
    s.push('b');

    println!("String with capacity: {}", s);

    //Assertion testing
    //Checking if left equals right
    assert_eq!(3, s.len()); // will return false

    //Can check Backtrace by typing
    //"export RUST_BACKTRACE=1"
    //"RUST_BACKTRACE=full" for more details
    //"unset RUST_BACKTRACE" to reset

    assert_eq!(2, s.len()); // won't show anything, but it passes
}