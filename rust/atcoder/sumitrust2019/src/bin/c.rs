fn main() {
    proconio::input! {
        x: usize,
    }

    if x >= 2000 {
        println!("1");
        return;
    }

    let mut dp: Vec<bool> = vec![false; x+1]; 
    let prices: Vec<usize> = vec![
        101,
        102,
        103,
        104,
        105,
    ];

    let len_p = prices.len();
    for i in 0..len_p {
        dp[prices[i]] = true; 
    }

    let mut i: usize = 101;
    while i <= x {
        if dp[i] {
            for p in 0..len_p {
                if i + prices[p] < x+1 {
                    dp[i + prices[p]] = true; 
                }
            } 
        }
        i += 1;
    }

    if dp[x] {
        println!("1");
    }else{
        println!("0");
    }

}