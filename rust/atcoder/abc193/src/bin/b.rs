fn main() {
    proconio::input! {
        n: usize,
        store: [(i32, i32, i32); n],
    }

    let mut price_min = std::i32::MAX;

    for s in store {
        if s.2 - s.0 <= 0 {
            continue;
        }
        if s.1 < price_min {
            price_min = s.1;
        } 
    }
    if price_min == std::i32::MAX {
        println!("-1");
        return;
    }
    println!("{}", price_min);
}