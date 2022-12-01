use std::collections::btree_map::ValuesMut;
use std::{thread, time::Duration, vec};
use std::sync::mpsc;
use std::sync::Mutex;
fn main(){
    let m=Mutex::new(5);
    {
        let mut num=m.lock().unwrap();
        *num=6;
    }
    println!("m:{:?}",m);
}
