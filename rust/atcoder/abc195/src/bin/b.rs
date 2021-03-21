/// https://blog.hamayanhamayan.com/entry/2021/03/14/001917
/// 解説を見た
/// みかんの個数は最大W*1000個 = 10^6
/// O(n)で計算できるらしい

fn main() {
    proconio::input! {
        A: u32,
        B: u32,
        W: u32
    }
    let W = 1000 * W;

    let mut min: u32 = std::u32::MAX;
    let mut max: u32 = 0;
    for i in 1..W+1 {
        let L = i * A;
        let R = i * B;
        if (L <= W && W <= R){ 
            min = std::cmp::min(min, i);
            max = std::cmp::max(max, i);
        }
    }

    if min == std::u32::MAX {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min, max);
    }
}
