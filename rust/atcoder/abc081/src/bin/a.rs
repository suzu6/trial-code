// https://doc.rust-lang.org/std/primitive.str.html#method.chars
// std::str::Chars
// s.chars()
// https://qiita.com/tubo28/items/e6076e9040da57368845

fn main() {
    proconio::input! {
        s: String
    }
    println!("{}", s.chars().filter(|&c| c == '1').count());
}
