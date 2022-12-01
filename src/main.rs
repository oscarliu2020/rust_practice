use List::{Cons,Nil};
use std::borrow::Borrow;
use std::rc::{Rc,Weak};
use std::cell::RefCell;
use std::vec;
#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil,
}
impl List{
    fn tail(&self)->Option<&RefCell<Rc<List>>>{
        match self {
            Cons(_,item)=>Some(item),
            Nil=>None,
        }
    }
}
impl Drop for List
{
    fn drop(&mut self) {
        println!("Drop");
    }
}
fn main(){
    
    {
        let a=Rc::new(Cons(0,RefCell::new(Rc::new(Nil))));
        let b=Rc::new(Cons(1,RefCell::new(Rc::clone(&a))));
        println!("{:?}",Rc::strong_count(&a));
        if let Some(link)=a.tail(){
            *link.borrow_mut()=Rc::clone(&b);
        }
        //println!("{} {}",Rc::strong_count(&a),Rc::strong_count(&b));
    }
    let y=RefCell::new(Weak::new());
    let out;
    {
        let x=Rc::new(vec![1]);
        *y.borrow_mut()=Rc::downgrade(&x);
        out =y.borrow().upgrade();
        println!("strong {}, weak {}",Rc::strong_count(&x),Rc::weak_count(&x));
        let z=y.borrow().upgrade().unwrap();
        println!("strong {}, weak {}",Rc::strong_count(&x),Rc::weak_count(&x));
    }
    println!("strong {}, weak {}",y.borrow().strong_count(),y.borrow().weak_count());
    {
        #[derive(Debug)]
        struct Node{
            value:i32,
            parent:RefCell< Weak<Node>>,
            child: RefCell<Vec<Rc<Node>>>,
        }
        let leaf=Rc::new(Node{
            value:10,
            parent:RefCell::new(Weak::new()),
            child:RefCell::new(vec![]),
        });
        let branch=Rc::new(Node{
            value:5,
            parent:RefCell::new(Weak::new()),
            child : RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut()=Rc::downgrade(&branch);
        println!("{:#?}",leaf.parent.borrow().upgrade());

    }
}