// https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reverse

use std::iter;
use std::char;

fn main() {
    proconio::input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    a.reverse(); 
    
    // println!("{:?}", a);
    let a = a;
    let alice_sum = a.iter()
        .enumerate()
        .filter(|&(i, x)| i % 2 == 0)
        .map(|(i, x)| x)
        .sum::<i32>();
    let bob_sum = a.iter()
        .enumerate()
        .filter(|&(i, x)| i % 2 == 1)
        .map(|(i, x)| x)
        .sum::<i32>();
    
    println!("{}", alice_sum - bob_sum);
}
