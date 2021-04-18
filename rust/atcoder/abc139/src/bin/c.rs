fn main() {
    proconio::input! {
        n: usize,
        h: [i32; n],
    }

    let mut max = 0;
    let mut count = 0;

    for i in 1..n {
        if h[i] <= h[i-1] {
            count += 1;
        }else {
            count = 0;
        }
        if max < count {
            max = count;
        }
    }

    println!("{}", max);
}