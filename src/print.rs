pub fn run() {
    println!("Hello from the print.rs file");

    println!("Number: {}", 1);

    println!("{} is from {}", "Yuichi", "Okinawa");

    println!(
        "{0} is from {1} and {0} likes to {2}",
         "Yuichi", "Okinawa", "golf"
    );

    println!(
        "{name} likes to play {activity}",
        name = "Yuichi", activity = "golf"
    );

    println!(
        "Binary: {:b}, HEX: {:x}, Octal: {:o}",
        10, 10, 10
    );

    //placeholder for debug trait
    println!("{:?}", (12, false, "herrro"));

    //simple math
    println!("10 * 10 = {}", 10*10);
}