use std::{thread, time::Duration};

fn main(){
    let handle =thread::spawn(||{
        for i in 1..10{
            println!("from thread:{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
}