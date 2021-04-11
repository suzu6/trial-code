fn main() {
    proconio::input! {
        n: usize,
        v: [u32; 2*n],
    }

    let mut v = v.clone();
    let mut s_v = v.clone();
    s_v.sort();
    // println!("{:?}", v);
    // println!("{:?}", s_v);

    let mut centor = s_v[n];
    // println!("centor {}", centor);

    let mut sum:u32 = 0;
    let mut j:usize = 0;
    
    while v.len() > 0 {
        // println!("{:?}", v);
        let i = v.len() / 2 - 1;
        if v[i] >= centor || v[i+1] >= centor {
            // もし片方でも中央値以上なら大きい方を取る
            if v[i] < v[i+1] {
                sum += v[i+1];
                v.remove(i+1);
                v.remove(i);
            }else {
                sum += v[i];
                v.remove(i+1);
                v.remove(i);
            }
            continue;
        }

        // 両方とも小さい場合

        // 中央値以上の値を探してくる
        while j < v.len() {
            if v[j] >= centor {
                sum += v[j];
                v.remove(j);
                v.remove(i);
                if j > i {
                    j -= 1;
                }
                break;
            }
            j += 1;
        }
        if i == v.len() / 2 - 1 {
            // j で中央値以上がない場合
            // 中央値を更新する
            s_v = v.clone();
            s_v.sort();
            centor = s_v[i];
            j = 0;
        }
        // println!("{} {}", i, sum);
    }
    println!("{}", sum);
}