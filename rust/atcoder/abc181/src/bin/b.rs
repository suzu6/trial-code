fn main() {
    proconio::input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in ab[i].0..=ab[i].1 {
            ans += j;
        }
    }

    println!("{}", ans);
  }