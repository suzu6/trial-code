fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n],
    }

    let mut sum = 0;
    for i in 0..n {
        sum += a[i] * a[i];
    }
    let res = sum * (n as i64);
    
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
    }
    let res = res - sum * sum;
    println!("{}", res);
}
