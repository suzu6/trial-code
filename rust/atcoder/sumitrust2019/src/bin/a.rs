fn main() {
    proconio::input! {
        m1: i32,
        d1: i32,
        m2: i32,
        d2: i32,
    }
    if m1 == m2 {
        println!("0");
    }else if d1 + 1 == d2{
        println!("0");
    }else{
        println!("1");
    }
}