fn main() {
    proconio::input! {
        a: u32,
        b: u32
    }
    if a*b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
