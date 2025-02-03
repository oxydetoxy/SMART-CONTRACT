use std::ptr;


fn main(){
    let mut x =String::from("hi");
    x.push_str("ishu");
    let  mut y = rahul(&mut x);
    println!("{x}");
}
fn rahul(q:&mut String){
    q.push_str("hing");
}