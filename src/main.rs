use std::collections::btree_map::ValuesMut;
use std::{thread, time::Duration, vec};
use std::sync::mpsc;
fn main(){
    let (tx,rx)=mpsc::channel();
    let hd=thread::spawn(move ||{
        let vals=vec!["a".to_string(),"b".to_string(),"c".to_string()];
        for val in vals
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
        
    });
    for recv in rx.iter(){
        println!("{recv}");
    }
}