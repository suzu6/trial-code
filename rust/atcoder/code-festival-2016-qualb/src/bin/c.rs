fn main() {
    proconio::input! {
        w: usize,
        h: usize,
        p: [i32; w],
        q: [i32; h],
    }
    
    let mut p = p;
    let mut q = q;
    p.sort();
    p.reverse();
    q.sort();
    q.reverse();
    
    let mut a = h as i32 + 1 ;
    let mut b = w as i32 + 1;
    let mut ret = 0;
    while p.len() > 0 || q.len() > 0 {
        let bp = match (p.last(), q.last()) {
            (Some(&pl), Some(&ql)) => pl < ql,
            (Some(_), None) => true,
            _ => false,
        };
        if bp {
            let pl = p.pop().unwrap();
            ret += a * pl;
            b -= 1;
        } else {
            let ql = q.pop().unwrap();
            ret += b * ql;
            a -= 1;
        }
    }
    println!("{}", ret)
}