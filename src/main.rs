#[derive(Debug,Clone, Copy)]
enum E{
    A,
    B
}
struct Cont{
    items: Vec<E>
}
impl Cont{
    fn take(&self,g:Option<E>)->E{
        g.unwrap_or_else(||self.func())
    }
    fn func(&self)->E{
        let mut cntA=0;
        let mut cntB=0;
        for i in &self.items{
            match i{
                E::A=>cntA+=1,
                E::B=>cntB+=1
            }
        }
        if cntA>cntB{
            E::A
        }else{
            E::B
        }
    }
}
fn main(){
    let x=Cont{
        items:vec![E::A,E::B,E::A,E::B,E::B]
    };
    let u=Some(E::A);
    let r=x.take(u);
    println!("{:?}",r);
    let u =None;
    let r=x.take(u);
    println!("{:?}",r);
}