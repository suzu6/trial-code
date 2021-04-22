fn main() {
    proconio::input! {
        H: i32,
        W: i32,
        h: i32,
        w: i32,
    }

    println!("{}", (H-h)*(W-w));
}