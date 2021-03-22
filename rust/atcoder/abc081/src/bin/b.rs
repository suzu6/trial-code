// https://doc.rust-lang.org/std/primitive.u32.html

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n]
    }
    println!("{}", a.iter()
        .map(|x| x.trailing_zeros())
        .min()
        .unwrap()
    );
}
