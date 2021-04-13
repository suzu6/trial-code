fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32
    }

    if c == 0 {
        if b >= a{
            println!("Aoki");
        }else{
            println!("Takahashi");
        }
    } else {
        if a >= b{
            println!("Takahashi");
        }else{
            println!("Aoki");
        }
    }
}