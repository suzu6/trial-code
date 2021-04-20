fn main() {
    proconio::input! {
        n: i32,
        r: i32,
    }

    if n >= 10 {
        println!("{}", r);
    }else{
        println!("{}", r + 100 * (10 - n));
    }
}