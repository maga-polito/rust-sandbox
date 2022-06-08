pub fn run(){
    // Print to console
    println!("Hello from the print.rs file");

    println!("Number: {}", 1);

    println!("{} is from {}", "Ana", "Venezuela");

    println!("{1} is one and {0} is zero", "0", "1");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity= "Baseball");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}