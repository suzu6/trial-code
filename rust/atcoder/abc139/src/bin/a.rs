fn main() {
    proconio::input! {
        s: String,
        t: String,
    }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut ans = 0;
    for i in 0..(s.len()) {
        if s[i] == t[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
