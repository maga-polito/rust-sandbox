// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block

pub fn run(){
    let name = "Ana";
    let mut age = 15;
    println!("My name is {} and I am {}", name, age);
    age=38;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple vars
    let (my_name, my_age)= ("Ana", 45);
    println!("My name is {} and I am {}", my_name, my_age);
}