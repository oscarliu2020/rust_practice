use std::thread;
use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
#[derive(Debug)]
struct Rect{
    h:u32,
    w:u32
}
struct F<T>{
    data:T,
}
trait Show{
    fn show(&self);
}
impl<T> Show for F<T>{
    fn show(&self){
        println!("show");
    }
} 
impl<T> Show for &F<T>
{
    fn show(&self){
        println!("show2");
    }
}
impl dyn Show{
    fn test(&self){
        println!("hello");
    }
}


fn main() {
    let mut list=vec![Rect{h:100,w:10},Rect{h:10,w:99},Rect{h:10,w:11}];
    let mut cnt=0;
    let  func=|r:&Rect| {r.h;cnt+=1;};
    list.sort_by_key(func);
    dbg!(&list,cnt);
}