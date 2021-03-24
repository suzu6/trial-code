fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    if a+b > c+d {
        println!("Left");
    }else if a+b == c+d {
        println!("Balanced");
    }else {
        println!("Right");
    }
}
