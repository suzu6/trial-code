fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32; m],
        a: [[i32; m]; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            sum += a[i][j] * b[j];
        }
        sum += c;

        if sum > 0 {
            ans += 1;
        } 
    }
    println!("{}", ans);
}