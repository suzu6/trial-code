
fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut min = 100000000;
    let mut max = 0;

    for i in 0..n {
        if min > b[i] {
            min = b[i];
        }
    }
    for i in 0..n {
        if max < a[i] {
            max = a[i];
        }
    }

    
    // println!("{} {}", min, max);
    if min - max >= 0 {
        println!("{}", min - max + 1);
    }else{
        println!("0");
    }
}
