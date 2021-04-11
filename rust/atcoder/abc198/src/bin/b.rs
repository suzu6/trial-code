fn main() {
    proconio::input! {
        n: String
    }

    let cs: Vec<char> = n.chars().rev().collect();
    // println!("{:?}", cs);

    let mut com: Vec<char> = vec![];
    let mut zero_flag = true;

    for i in 0..(cs.len()) {
        let c = cs[i];
        if c == '0' && zero_flag {
            // 末尾に続く0を無視
            continue;
        } else {
            zero_flag = false;
        }
        com.push(c);
    }

    if com.len() < 2 {
        println!("Yes");
    } else {
        let mut com_rev = com.clone();
        com_rev.reverse();

        let mut flg = true;
        for i in 0..(com.len()) {
            if com[i] != com_rev[i] {
                flg = false;
            }
        }

        if flg {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
