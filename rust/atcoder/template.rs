use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
  proconio::input! {
      n: usize,
      w: i32,
      h: i32,
      a: [i32; n],
      m: usize,
      xy: [(i32, i32); m],
      s: String,
  }

  let cs: Vec<char> = s.chars().collect();
}

fn hash(){
    let a = vec![1, 2, 3];
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
}