use std::{thread, time::Duration, vec};

fn main(){
    {
        let handle =thread::spawn(||{
        for i in 1..10{
            println!("from thread:{}",i);
            thread::sleep(Duration::from_millis(1));
        }
        });
        for i in 1..5{
            println!("from main:{}",i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }
    {
        let v=vec![1,2,3];
        let handle=thread::spawn( move ||{println!("{v:?}")});
        handle.join().unwrap();
    }
    
}