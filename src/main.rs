fn main(){
    let  x=String::from("hello");
    let y=Some(x);
    let r=&y;
    match r{
        Some(ref e)=>{},
        None=>{}
    }
    println!("{y:?}");
    {
        #[derive(Debug)]
        struct P{
            x:i32,
            y:i32,
            l:String
        }
        let p=P{
            x:1,
            y:2,
            l:"abc".to_string(),
        };
        match p {
            P{x:ref a,y:ref b,l: c}=>(),
            _=>{}
        }
    }
    {
    
    }
    
}
fn test((x,y):&mut (usize,String)){
    *x=1000;
    y.push('a');
}