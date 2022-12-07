use std::{path::Iter, ops::Add};
use std::ops::Deref;
pub trait Tr{
    fn  test();
    fn test2(&self){
        static mut  CNT:i32=0;
        unsafe{
            CNT+=1;
            println!("{CNT}");
        }
    }
}
pub struct T1();
pub struct T2();

impl Tr for T1{
    fn  test(){
        static mut CNT:i32=0;
        unsafe{
            CNT+=1;
            println!("{CNT}");
        }
    }
}
impl Tr for T2{
    fn  test(){
        static mut CNT:i32=0;
        unsafe{
            CNT+=1;
            println!("{CNT}");
        }
    }
}
struct Counter{
    cnt:u64,
    max:u64
}
impl Counter{
    fn new(m:u64)->Self{
        Self { cnt: 0, max: m}
    }
}
impl Iterator for Counter{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cnt{
             e if e==self.max=>{None},
            _=>{self.cnt+=1;Some(self.cnt)}
        }
    }
}
struct Adder{
    sum:i128
}
impl Add for Adder{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Adder{
            sum:self.sum+rhs.sum
        }
    }
}
impl<T> Add<T> for Adder
where T:Into<i128>{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Adder{
            sum:self.sum+ rhs.into()
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn work(){
        let x=T1();
        let y=T2();
        T1::test();
        T2::test();
        <T1 as Tr>::test();
        x.test2();
        y.test2();
    }
    #[test]
    fn test_iter(){
        let ci=Counter::new(10);
        for i in ci{
            println!("{i}");
        }
    }
    #[test]
    fn test_adder(){
        let a=Adder{sum:0};
        let a=a+3000i64;
        println!("{}",a.sum);
    }
}