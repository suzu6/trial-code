fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    if a+b >= 15 && b >= 8 {
        println!("1");
    }else if a+b >= 10 && b >= 3 {
        println!("2");
    }else if a+b >= 3 {
        println!("3");
    }else {
        println!("4");
    }
}
