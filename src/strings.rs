// Primitive str = Immutable fixed-lenght string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let mut hello = String::from("Hello ");

    // get lenght
    println!("Length: {}", hello.len());

    hello.push('w');

    hello.push_str("orld!");

    println!("{}", hello);

    println!("Capacity: {}", hello.capacity());

    println!("Empty? {}", hello.is_empty());

    println!("Contains: {}", hello.contains("world"));
}