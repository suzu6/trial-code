fn main() {
    proconio::input! {
        n: i32
    }

    if n == 1 {
        println!("0");
    }else{
        println!("{}", n-1);
    }
}
