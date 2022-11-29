fn main() {
    let v1=vec![1,2,3];
    let v2:Vec<_>=v1.iter().map(|x| x+1).collect();
    for (i,x) in v1.iter().enumerate(){
        println!("{i}: {x}");
    }
    for (i,x) in v2.iter().enumerate(){
        println!("{i}: {x}");
    }
}