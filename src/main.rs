fn main() {
    let  list = vec![1, 2, 3];
    let func=|| println!("{:?}",list);
    println!("{:?}",list);
    func();
    println!("{:?}",list);
}
