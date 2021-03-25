use std::cmp;

fn main() {
    proconio::input! {
        n: usize,
        ab: [(i32, i32); n],
    }

    let mut res = std::i32::MAX;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                res = cmp::min(res, ab[i].0 + ab[j].1);
            }else {
                res = cmp::min(res, cmp::max(ab[i].0, ab[j].1));
            }
        }
    }
    println!("{}", res);
    // ↓　2問通らなかった
    // let iab: Vec<(usize, usize, usize)> = ab.iter()
    //     .enumerate()
    //     .map(|(i, ab)| (i, ab.0, ab.1))
    //     .collect(); 

    // let mut va: Vec<_> = iab.clone().into_iter().collect();
    // let mut vb: Vec<_> = iab.clone().into_iter().collect();
    // va.sort_by(|x, y| x.1.cmp(&y.1));
    // vb.sort_by(|x, y| x.2.cmp(&y.2));
    // // println!("{:?}", va);
    // // println!("{:?}", vb);

    // if va[0].0 == vb[0].0 {
    //     let sum = va[0].1 + vb[0].2;
    //     let second = cmp::min(va[1].1, vb[1].2);
    //     println!("{}", cmp::min(sum, second));
    // }else {
    //     println!("{}", cmp::max(va[0].1, vb[0].2));
    // }
}
