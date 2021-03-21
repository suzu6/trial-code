/// 
/// 解説 https://atcoder.jp/contests/abc196/editorial/930
/// 

fn dfs() {
    
}

fn main() {
    proconio::input! {
        H: u32,
        W: u32,
        A: u32,
        B: u32
    }
    let mut dmap = vec![0; W*H];
    // for x in dmap {
    //     println!("{:?}", x);
    // }
    let result = rec(A, W, 0, W*H, dmap.clone());
    println!("{}", result);
}
