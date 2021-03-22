
use std::i32;

fn search(n: i32, y: i32) -> (i32, i32, i32){
    for i in 0..n+1 {
        for j in 0..n+1-i {
            let k = n - i - j;
            let total = 10000 * i + 5000 * j + 1000 * k;
            if total == y {
                return (i, j, k);
            }
        }
    }
    (-1, -1, -1)
}

fn main() {
    proconio::input! {
        n: i32,
        y: i32,
    }
    let (i, j, k) = search(n, y);
    println!("{} {} {}", i, j, k);
}
