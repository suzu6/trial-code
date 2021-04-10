fn main() {
    proconio::input! {
        v: i32,
        t: i32,
        s: i32,
        d: i32,
    }
    if d < v * t || v * s < d {
        println!("Yes");
    } else {
        println!("No");
    }
}
