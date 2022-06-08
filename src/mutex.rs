use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn run(){
    let state = Arc::new(Mutex::new(Vec::<String>::new()));
    let h: Vec<JoinHandle<()>> = (1..=2).map(|_tid|{
        let state = state.clone();
        thread::spawn(move || {
            for i in 1..5{
                state.lock().unwrap().push(i.to_string().add("-").add(_tid.to_string().as_str()));
            }
        })
        }).collect();
        h.into_iter().for_each(|h|{h.join().unwrap();});
        print!("=============");
        state.lock().unwrap().iter().for_each(|s|{println!("{}", s);});
}