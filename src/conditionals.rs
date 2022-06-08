pub fn run(){
    let age = 18;
    let check_id: bool = false;

    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id{
        println!("Bartender: Sorry you have to leave");        
    } else {
        println!("Bartender: I'll need to see your ID")
    }
}