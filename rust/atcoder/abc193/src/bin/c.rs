
use std::collections::HashSet;

fn main() {
    proconio::input! {
        n: u128,
    }
    let mut array: Vec<u128> = vec![];
    for b in 2..100 {
        for a in 2..n {
            if num::pow(a, b) <= n {
                // println!("{}", num::pow(a, b));
                array.push(num::pow(a, b));
            }else {
                break;
            }
        }
    }
    let uniq: HashSet<u128> = array.clone().into_iter().collect();
    // println!("{}", array.len() as u128);
    // println!("{}", uniq.len() as u128);
    println!("{}", n - uniq.len() as u128);
}