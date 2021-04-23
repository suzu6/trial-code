fn main() {
    proconio::input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut ans = false;
    for i in 0..n {
        let x1 = xy[i].0;
        let y1 = xy[i].1;
        for j in (i+1)..n{
            let x2 = xy[j].0;
            let y2 = xy[j].1;
            for k in (j+1)..n{
                let x3 = xy[k].0;
                let y3 = xy[k].1;
                if (x2 -x1)*(y3-y1) == (y2-y1)*(x3-x1){
                    ans = true;
                }
            }
        }
    }

    if ans {
        println!("Yes");
    }else{
        println!("No");
        
    }
  }