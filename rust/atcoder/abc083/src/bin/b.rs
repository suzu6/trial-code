use std::iter;
use std::char;

fn main() {
    proconio::input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let sum = (1..n+1).filter(|x| {
        let keta_sum = x.to_string()
            .chars()
            .map(|c| (c as u8 - b'0') as u32)
            .sum::<u32>();
        a <= keta_sum && keta_sum <= b
    }).sum::<u32>();

    println!("{}", sum)
}
