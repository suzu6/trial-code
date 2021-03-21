
fn main() {
    proconio::input! {
        M: u32,
        H: u32
    }
    if H % M == 0 {
        println!("Yes");
    }else{
        println!("No");
    }
    // println!("{} {}", M, H);
}
