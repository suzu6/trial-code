/// sum((x_i - p)^2) = sum( p^2 - 2(sum(x_i))p + sum(x_i^2) )
/// 微分すると
/// sum(2p -2(sum(x_i))) = 2Np -2(sum(x_i)) = 0
/// p = sum(x_i) / N = average
/// よってpはXの平均
/// 整数である必要があるので 保険として平均の前後２つづつの整数を調べて最小のものとする

fn main() {
    proconio::input! {
        n: usize,
        x: [i32; n],
    }

    let sum: i32 = x.iter().sum();
    let p = sum / (n as i32);

    let mut ans: i32 = 1000000000;
    
    for k in -2..4 {
        let q = p + k;
        let mut ave = 0;
        for i in 0..n {
            ave += (x[i] - q) * (x[i] - q); 
        }
        if ave < ans {
            ans = ave;
        }
        // println!("{} {}", q, ave);
    }
    println!("{}", ans);
}