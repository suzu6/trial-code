fn main() {
    proconio::input! {
        n: usize,
    }

    let ans = n * (n - 1) / 2;

    println!("{}", ans);
}
