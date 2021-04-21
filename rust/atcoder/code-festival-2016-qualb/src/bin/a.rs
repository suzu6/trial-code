fn main() {
    proconio::input! {
        s: String,
    }
    let r = "CODEFESTIVAL2016";
    let cs:Vec<char> = s.chars().collect();
    let cr:Vec<char> = r.chars().collect();

    let mut ans = 0;
    for i in 0..cs.len() {
        if cs[i] != cr[i] {
            ans += 1;
        }
    }
    // let dst: Vec<String> = cs.iter().map(|x| x.to_string()).collect();
    println!("{}", ans);
}