fn main() {
    proconio::input! {
        N: u64
    }
    let mut N = N;
    for i in (1..13).rev() {
        let keta = std::num::pow(10, i);
        if N < keta {
            continue;
        }
        
    }
}
