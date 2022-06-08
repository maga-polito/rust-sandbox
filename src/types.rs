pub fn run(){
    // Default is "i32"   
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 1231231231231;


    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    let a1 = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (x,y,z,is_active, a1, face));
    

}