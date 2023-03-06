pub fn run(){
    greeting("Wasssuuup!", "Yuichi");

    //Bind function  values to variable
    let get_sum = add(10, 25);
    println!("Sum: {}", get_sum);

    //Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_nums(15, 1));
}

fn greeting(greet: &str, name: &str) {
    println!("Hello {}, {}", name, greet);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 // don't use semicolon
}