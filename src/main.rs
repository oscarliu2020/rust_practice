use core::slice;
fn main(){
    {    
        let mut num=5;
        let r1=&num as *const i32;
        let r2=&mut num as *mut i32;
        unsafe{
            dbg!(*r2);
        }
    }
    {
        let address = 0x012345usize;
        let r = address as *const i32;
        dbg!(r);
    }
    {
        let mut  a=[1,2,3,4,5];
        let (s1,s2)=split_at_mut(&mut a, 3);
        for i in s1{
            print!("{i} ");
            *i+=1;
        }
        println!("");
        for i in s2{
            print!("{i} ")
        }
        println!("");
        for i in &a{
            print!("{i} ")
        }
        println!("");
    }
    {
        let address=0x012345usize;
        let r=address as *mut i32;
        let values:&[i32]=unsafe {
            slice::from_raw_parts_mut(r,1,)
        };
    }
    let r={
        static mut CNT:i32=0;
        unsafe{
            CNT+=1;
            println!("{}",CNT)
        }
        
    };

}
fn split_at_mut(val: &mut [i32],mid:usize)->(&mut [i32],&mut [i32]){
    let len=val.len();
    let ptr=val.as_mut_ptr();
    assert!(mid<=len);
    unsafe{
        (slice::from_raw_parts_mut(ptr, mid),slice::from_raw_parts_mut(ptr.add(mid), len-mid))
    }
}