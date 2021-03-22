use std::str;

fn main() {
    proconio::input! {
        s: String
    }
    // 逆から探す
    let s: Vec<char> = s.chars().rev().collect();

    let patterns: Vec<Vec<char>> = [
        "dream",
        "dreamer",
        "erase",
        "eraser"]
        .iter()
        .map(|x| x.chars().rev().collect())
        .collect();
    
    let mut s = &s[..];
    let mut successed = true;

    while s.len() > 0 {
        let matched = patterns.iter()
            .find(|&p| s.starts_with(p));
        if let Some(p) = matched {
            s = &s[p.len()..];
        } else {
            successed = false;
            break;
        }
    }

    println!("{}", if successed { "YES" } else { "NO" });
}
