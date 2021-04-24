fn main() {
    proconio::input! {
        n: usize,
        s: String,
        q: usize,
        tab: [(usize, i32, i32); q],
    }

    let mut cs: Vec<char> = s.chars().collect();
    
    let mut ai = 0;
    let mut bi = 0;

    let mut sei = true;
    for i in 0..q {
        let t = tab[i];
        if t.0 == 1 {
            if sei {
                ai = t.1-1;
                bi = t.2-1;
            }else{
                ai = t.1-1 - n as i32;
                bi = t.2-1 - n as i32;
                if ai < 0 {
                    ai = ai + 2 * n as i32;
                }
                if bi < 0 {
                    bi = bi + 2 * n as i32;
                }
            }

            let tmp = cs[ai as usize];
            cs[ai as usize] = cs[bi as usize];
            cs[bi as usize] = tmp;
        }else{
            sei = !sei;
        }

    }
    if !sei {
        let mut later = cs.split_off(n);
        later.append(&mut cs);
        cs = later;  
    }
    
    let pb: Vec<String> = cs.iter().map(|x| x.to_string()).collect();

    println!("{}", pb.join(""));
}
