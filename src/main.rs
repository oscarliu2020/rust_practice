use std::collections::btree_map::ValuesMut;
use std::{thread, time::Duration, vec};
use std::sync::mpsc;
fn main(){
    let (tx,rx)=mpsc::channel();
    let tx1=tx.clone();
    let hd=thread::spawn(move ||{
        let vals=vec!["a".to_string(),"b".to_string(),"c".to_string()];
        for val in vals
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
        
    });
    let hd1=thread::spawn(move ||{
        let vals=vec!["d".to_string(),"e".to_string(),"f".to_string()];
        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    for recv in rx.iter(){
        println!("{recv}");
    }
}