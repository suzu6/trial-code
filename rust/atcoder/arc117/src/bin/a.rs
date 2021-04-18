fn main() {
    proconio::input! {
        a: usize,
        b: usize,
    }

    let mut va: Vec<i32> = vec![0; a];
    let mut vb: Vec<i32>= vec![0; b];

    if a > b {
        for i in 0..a {
            va[i] = (i + 1) as i32;
            let mut j = i;
            if j > b-1 {
                j = j % b;
            }
            vb[j] -= (i + 1) as i32;
        }
    }else {
        for i in 0..b {
            vb[i] = (-1) * (i + 1) as i32;
            let mut j = i;
            if j > a-1 {
                j = j % a;
            }
            va[j] += (i + 1) as i32;
        }
    }
    
    // 要素を整数型から文字列型変換
    let ca: Vec<String> = va.iter().map(|x| x.to_string()).collect();
    let cb: Vec<String> = vb.iter().map(|x| x.to_string()).collect();
    // 文字列型vectorを区切り文字で結合
    println!("{} {}", ca.join(" "), cb.join(" "));
}
