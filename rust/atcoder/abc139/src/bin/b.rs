fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }

    let mut plugs = 1;
    let mut ans = 0;

    while b > plugs {
        plugs += a - 1;
        ans += 1;
    }

    println!("{}", ans);
}