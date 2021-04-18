fn main() {
    proconio::input! {
        n: usize,
        s: String,
    }

    let mut cs: Vec<char> = s.chars().collect();
    // println!("{:?}", cs);

    for i in (0..n).rev() {
        let mut pre = cs[0];
        for j in 1..(i+1) {
            // println!("{} {}", i, j);
            if cs[j] == pre {
                cs[j-1] = pre;
                pre = cs[j];
            } else {
                let tmp = cs[j];
                cs[j-1] = se(pre, cs[j]);
                pre = tmp;
            }
        }
        // println!("{:?}", cs);
    }
    println!("{}", cs[0]);
}

fn se(a: char, b: char)->char{
    if a == 'B' {
        if b == 'W' {
            return 'R';
        }else {
            return 'W';
        }
    } else if a == 'W' {
        if b == 'B' {
            return 'R';
        }else {
            return 'B';
        }
    } else {
        if b == 'B' {
            return 'W';
        }else {
            return 'B';
        }
    }
}
