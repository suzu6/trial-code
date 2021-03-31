fn main() {
    proconio::input! {
        a: f64,
        b: f64,
    }
    println!("{}", ((a-b)*100.0)/a);
}