fn main() {
    proconio::input! {
        a: [i32; 9 as usize],
        n: usize,
        b: [i32; n],
    }

    for i in 0..n {
        for j in 0..9 {
            if a[j] == b[i] {
                a[j] == -1;
            }
        }
    }

    let mut ans = "No";

    for i in 0..3 {
        if a[i] == -1 && a[i + 1] == -1 && a[i + 2] == -1 {
            ans = "Yes";
        }
    }
    for i in 0..3 {
        if a[i] == -1 && a[i + 3] == -1 && a[i + 6] == -1 {
            ans = "Yes";
        }
    }
    
    if a[0] == -1 && a[4] == -1 && a[8] == -1 {
        ans = "Yes";
    }
    if a[2] == -1 && a[4] == -1 && a[6] == -1 {
        ans = "Yes";
    }
    
    println!("{}", ans);
}
