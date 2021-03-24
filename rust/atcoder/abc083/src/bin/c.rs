fn main() {
    proconio::input! {
        x: u64,
        y: u64,
    }
    let mut count = 0;
    let mut a = x; 
    while a <= y {
        count += 1;
        a *= 2;
    }
    println!("{}", count);
}
