fn main() {
    proconio::input! {
        n: usize,
        s: i32,
        d: i32,
        magics: [(i32, i32); n]
    }

    let mut flg = false;
    for magic in magics {
        if magic.0 < s && magic.1 > d {
            flg = true;
        }
    }

    if flg {
        println!("Yes");
    }else{
        println!("No");
    }
}