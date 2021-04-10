use proconio::{input, marker::*};
use std::*;

fn main() {
    input! {
        chars: Chars,
    }
    let mut isYes = false;
    for i in 0..chars.len() {
        if i % 2 == 0 {
            if !chars[i].is_uppercase() {
                isYes = true;
            }else{
                isYes = false;
            }
        } else {
            if chars[i].is_uppercase() {
                isYes = true;
            }else{
                isYes = false;
            }
        }
        if !isYes {
            break;
        }
    }
    if isYes {
        println!("Yes");
    } else {
        println!("No");
    }
}
