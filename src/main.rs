
enum List<T>{
    Cons(T,Rc<List<T>>),
    NIL
}
struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new (x:T)->Self{
        Self(x)
    }
}
impl<T> Drop for MyBox<T>{
    fn drop(&mut self) {
        println!("smart pointer drop");
    }
}
impl<T> Deref for MyBox<T>
{
    type Target = T;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}
use std::{ops::Deref, fmt::Display, rc::Rc, cell::RefCell};

use List::{Cons,NIL};
fn hello(a:&str){
    println!("hello, {}",a);
}
fn main() {
    {
        let x=5;
        let y=MyBox::new(x);
        assert_eq!(x,*y);

    }
    {
        let m=MyBox::new(String::from("world"));
        hello(&m);
        drop(m);
        let q=5;
        println!("{q:?}");
    }
    
    {
        let a=Rc::new(Cons(5,Rc::new(NIL)));
        println!("{}",Rc::strong_count(&a));
        let b=Cons(3,Rc::clone(&a));
        println!("{}",Rc::strong_count(&a));
        {
            let b=Cons(3,Rc::clone(&a));
            println!("{}",Rc::strong_count(&a));
        }
        println!("{}",Rc::strong_count(&a));
    }
}
#[cfg(test)]
mod tests{
    use std::cell::RefCell;

    trait Msg{
        fn send(&self,msg:&str);
    }
    struct Msgq{
        sets:RefCell<Vec<String>>
    }
    impl Msgq{
        fn new()->Self{
            Self { sets: RefCell::new(vec![]) }
        }

    }
    impl Msg for Msgq{
        fn send(&self,msg:&str) {
            self.sets.borrow_mut().push(msg.to_string());
        }
    }
    #[test]
    fn test(){
        let x =Msgq::new();
        x.send("q");
    }
}