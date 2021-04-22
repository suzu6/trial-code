fn main() {
    proconio::input! {
        n: usize,
        m: i64,
        ab: [(i64, i64); n],
    }

    let mut ab = ab;
    ab.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // println!("{:?}", ab);

    let mut m = m;
    let mut ans = 0;
    for i in 0..n {
        if ab[i].1 <= m {
            ans += ab[i].0 * ab[i].1;
            m = m - ab[i].1;
        }else{
            ans += ab[i].0 * m;
            break;
        }
    }
    println!("{}", ans);
}