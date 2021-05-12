fn main() {
    proconio::input! {
        s: String,
    }
    let mut cs: Vec<char> = s.chars().collect();

    // let tmp = cs[0];
    // cs[0] = cs[1];
    // cs[1] = cs[2];
    // cs[2] = tmp;

    // 上の処理と同じ
    cs.rotate_left(1);

    let pb: Vec<String> = cs.iter().map(|x| x.to_string()).collect();
    println!("{}", pb.join(""));
}
