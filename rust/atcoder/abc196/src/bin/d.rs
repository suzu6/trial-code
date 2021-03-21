fn rec(a: usize, w: usize, i: usize, l: usize, dmap: &mut Vec<usize>) -> usize{
    if a == 0{
        return 1;
    }
    if i == l {
        return 0;
    }
    let mut count = 0;
    for x in i..l {
        if dmap[x] == 1{
            continue;
        }
        if x+1 % w == 0 && dmap[x+1] != 1{
            let cmap = dmap.clone();
            cmap[x] = 1;
            cmap[x+1] = 1;
            count += rec(a-1, w, i+2, l, cmap);
        }
        if x+w < l && dmap[x+w] != 1{
            let cmap = dmap.clone();
            cmap[x] = 1;
            cmap[x+w] = 1;
            count += rec(a-1, w, i+1, l, cmap);
        }
        dmap[x] == 1;
    }
    return count;
}


fn main() {
    proconio::input! {
        H: usize,
        W: usize,
        A: usize,
        B: usize
    }
    let mut dmap = vec![0; W*H];
    // for x in dmap {
    //     println!("{:?}", x);
    // }
    let result = rec(A, W, 0, W*H, dmap.clone());
    println!("{}", result);
}
