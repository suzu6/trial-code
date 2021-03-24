use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    proconio::input! {
        n: i32,
        k: i32,
        a: [i32; n]
    }
    // ユニークな数
    let uniq: HashSet<i32> = a.clone().into_iter().collect();
    let len = uniq.len() as i32 - k;
    if len <= 0 {
        println!("0");
        return;
    }
    // 要素ごとのカウント
    let mut map = HashMap::new();
    for x in a {
        let counter = map.entry(x).or_insert(0);
        *counter += 1;
    }
    // カウント順にソート
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x, y| x.1.cmp(&y.1));
    v.reverse();
    // println!("{:?}", v);
    let mut answer = 0;

    // 数が小さいものからk以下まで足し算
    for _ in 0..len {
        answer += v.pop().unwrap().1;
    }
    println!("{}", answer);

    // 最小を取得
    // let sorted_keys = map2.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
}
