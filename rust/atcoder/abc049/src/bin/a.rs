use std::char;

fn main() {
    proconio::input! {
        c: char
    }

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let mut answer = "consonant";
    for v in vowels {
        if c == v {
            answer = "vowel";
            break;
        }
    }
    println!("{}", answer);
}
