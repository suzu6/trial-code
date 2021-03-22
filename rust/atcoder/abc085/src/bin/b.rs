
use std::u32;

fn main() {
    proconio::input! {
        n: usize,
        mut d: [u32; n],
    }
    d.sort();
    d.reverse(); 
    
    let mut count = 0;
    let mut min = u32::MAX;

    for x in d {
        if x < min {
            count += 1;
            min = x;
        } 
    }
    
    println!("{}", count);
}
