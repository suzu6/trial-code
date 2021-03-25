fn main() {
    proconio::input! {
        n: usize,
        t: [i32; n],
        m: usize,
        px: [(usize, i32); m],
    }
    // println!("{:?}", t);
    // println!("{:?}", px);

    let sum: i32 = t.iter().sum();
    // println!("{}", sum);

    for i in 0..m {
        println!("{}", sum + (px[i].1 - t[px[i].0 - 1]));
    }
}
