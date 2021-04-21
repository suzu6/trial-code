fn main() {
    proconio::input! {
        n: usize,
        a: i32,
        b: i32,
        s: String,
    }
    
    let cs:Vec<char> = s.chars().collect();

    let mut count_a = 0;
    let mut count_b = 0;

    for i in 0..cs.len() {
        let c = cs[i];
        if c == 'c' {
            println!("No");
            continue;
        }
        if c == 'a' {
            if count_a + count_b < a + b {
                println!("Yes");
                count_a += 1;
                continue;
            }else{
                println!("No");
            }
        }else{
            if count_a + count_b < a + b && count_b < b {
                println!("Yes");
                count_b += 1;
                continue;
            }else{
                println!("No");
            }

        }
    }


}