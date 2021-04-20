fn main() {
    proconio::input! {
        n: i32,
        k: i32,
    }

    let mut ans = 1;
    let mut n = n;

    while n >= k {
        ans += 1;
        n = n / k;
    }
    println!("{}", ans);
}