use std::collections::HashMap;

fn main() {
    proconio::input! {
        s: String,
    }
    const RADIX: u32 = 10;
    
    let cs: Vec<u32> = s.chars().map(|x| x.to_digit(RADIX).unwrap()).collect();
    let n = cs.len();

    if n == 1 {
        if cs[0] % 8 == 0 {
            println!("Yes");
            return;
        }
        println!("No");
        return;
    }
    
    if n == 2{
        if (cs[0] + cs[1]*10) % 8 == 0 {
            println!("Yes");
            return;
        }
        if (cs[1] + cs[0]*10) % 8 == 0 {
            println!("Yes");
            return;
        }
        println!("No");
        return;
    }

    if is_all_odd(cs.clone()) {
        // 全て奇数の場合は8の倍数になりえない
        println!("No");
        return;
    }

    
    // 要素ごとのカウント
    let mut map = HashMap::new();
    for i in 0..n {
        let counter = map.entry(cs[i]).or_insert(0);
        if *counter < 3 {
            // 3桁しか確認しないため
            *counter += 1;
        } 
    }
    let mut cs: Vec<u32> = vec![];
    for (k, v) in &map {
        for _ in 0..(*v as usize){
            cs.push(*k);
        }
    }
    let n = cs.len();
    // println!("{:?}", cs);


    // 8の倍数は下三桁のみの確認で良い　1000 / 25 = 8 
    for i in 0..n {
        if cs[i] % 2 == 1 {
            continue;
        }
        for j in 0..n {
            if i == j {
                continue;
            }
            for k in 0..n {
                if i == k || j == k{
                    continue;
                }
                // println!("{} {}", cs[i] + cs[j]*10 + cs[k]*100, (cs[i] + cs[j]*10 + cs[k]*100) % 8);
                if (cs[i] + cs[j]*10 + cs[k]*100) % 8 == 0 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}

fn is_all_odd(cs: Vec<u32>) -> bool {
    for c in cs{
        if c % 2 == 0 {
            return false;
        }
    }
    return true;
}