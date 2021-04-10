fn main() {
    proconio::input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }

    let vec:Vec<String> = a.iter()
        .filter(|&e| e != &x)
        .cloned()
        .map(|x| x.to_string())
        .collect();
    print!("{}", vec.join(" "));
}
