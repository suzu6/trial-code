fn isGt(c: char) -> bool {
    if c == '<' {
        return true;
    } else {
        return false;
    }
}

fn main() {
    proconio::input! {
        n: usize,
        s: String,
        a: [i32; n+1],
    }

    println!("{:?}", a);
    let cs: Vec<char> = s.chars().collect();
    let mut max_nest = 0;
    let mut tmp = 1;
    for i in 0..(n-1) {
        if cs[i] == cs[i+1] {
            tmp += 1;
        }else{
            tmp = 1;
        }
        if max_nest < tmp {
            max_nest = tmp;
        }
    }

    println!("{}", max_nest);
}
