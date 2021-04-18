fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
    }

    let e = 1000000007;
    let mut a: Vec<u64> = a.clone();
    a.sort();

    let mut pre = 0;
    let mut ans = 1;
    for i in (0..n).rev() {
        if pre == 0 {
            pre = a[i];
            continue;
        }
        if pre == a[i] {
            pre = a[i];
            continue;
        }
        ans = ans * (pre - a[i] + 1);
        ans = ans % e;
        pre = a[i];
        
        // println!("{} {}", a[i], ans % e);
    }
    ans = (ans * (a[0] + 1)) % e;
    // println!("{:?}", a);
    println!("{}", ans % e);
}
