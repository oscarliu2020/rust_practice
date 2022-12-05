use std::{sync::{Mutex,Arc}, thread};
use std::time::Duration;
use std::rc::Rc;
fn main(){
    let counter=Arc::new(Mutex::new(0));
    let mut  handles=vec![];
    for  _ in 0..10{
        let counter=Arc::clone(&counter);
        let handle=thread::spawn(move ||{
            let mut  num=counter.lock().unwrap();
            *num+=1;

        });
        handles.push(handle);
    }
    for h in handles{
        h.join().unwrap();
    }
    println!("{}",*counter.lock().unwrap());
    let q=vec![1,2,3];
    let x:i32=q.iter().sum();
}