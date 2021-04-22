fn main() {
    proconio::input! {
        a: i64,
        b: i64,
    }
    // if a == 0 {
    //     println!("{}", f(b));
    // }else{
    // }
    println!("{}", f(b) ^ f(a-1));

    // let mut ss = 0;
    // for i in a..=b{
    //     ss = ss ^ i;
    // }
    // println!("{}", ss);
}

fn f(a: i64) -> i64 {
    let cnt = (a + 1) / 2;
    let mut result = cnt % 2;
    if a % 2 == 0 {
        result ^ a
    } else {
        result
    }
}