fn main() {
    proconio::input! {
        n: f64,
    }

    let mut ans = 0;
    for i in 1..50000 {
        if num::Float::floor(i as f64 * 1.08) == n {
            ans = i
        }
    }
    if ans == 0 {
        println!(":(");
    }else{
        println!("{}", ans);
    }
}