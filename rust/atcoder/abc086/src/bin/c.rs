use std::i32;

fn main() {
    proconio::input! {
        n: usize,
        a: [[i32; 3]; n]
    }

    let mut p_t = 0;
    let mut p_x1 = 0;
    let mut p_x2 = 0;

    let mut successed = true;

    for b in a {
        let t = b[0] - p_t;
        let x1 = b[1] - p_x1;
        let x2 = b[2] - p_x2;

        let distance = t - (i32::abs(x1) + i32::abs(x2));
        // 時間が足りないとき
        // または、余った時間が奇数の時はその点に居られない。
        if distance < 0 || distance % 2 == 1 {
            successed = false;
            break;
        }

        p_t = b[0];
        p_x1 = b[1];
        p_x2 = b[2];
    }
    println!("{}", if successed { "Yes" } else { "No" });
}
