pub fn run(){
    let age: i8 = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("I need to see your ID");
    }
    //shoft_hand
    let is_of_age: bool = if age > 21 { true } else { false };
    println!("Is of age: {}", is_of_age);

}