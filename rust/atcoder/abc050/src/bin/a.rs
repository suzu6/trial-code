fn main() {
    proconio::input! {
        a: i32,
        op: char,
        b: i32,
    }
    if op == '+' {
        println!("{}", a+b);
    }else{
        println!("{}", a-b);
    }
}
