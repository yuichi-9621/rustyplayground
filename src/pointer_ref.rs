//Reference pointers - Point to a resource in memory

pub fn run(){
    let arr1 = [1, 2, 3, 4];
    let arr2 = &arr1;

    println!("Refference Array Values: {:?}", (arr1, arr2));

    let vec1: Vec<i32> = vec![1, 2, 3, 4];
    println!("Vector Values: {:?}", vec1);

    let mut vec2: Vec<i32> = (*vec1).to_vec();

    let vec5 = &vec1;
    println!("Check refference vec: {:?}", vec5);

    let vec3 = vec1;

    //Can't use vec1 again since it has assinged to vec3
    //println!("Check vec1 again: {:?}", vec1);


    vec2.push(5);

    let vec4 = &vec2;

    println!("New Vector Values: {:?}", vec2);

    println!("Vec3: {:?}", vec3);

    println!("Vec4: {:?}", vec4);

}