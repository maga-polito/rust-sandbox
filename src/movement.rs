pub fn run(){
    let mut s1 = "Hello Ana".to_string();

    println!("s1: {}", s1);

    let s2 = s1;

    println!("s2: {}", s2);

    s1 = "new magic value".to_string();

    println!("s1: {}", s1);

}