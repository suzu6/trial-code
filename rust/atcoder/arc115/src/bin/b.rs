fn main() {
    proconio::input! {
        N: usize,
        C: [[i32; N]; N]
    }

    let mut cmin = std::i32::MAX;
    let mut ci: usize = 0;
    let mut cj: usize = 0;
    for i in 0..N {
        for j in 0..N {
            if C[i][j] < cmin{
                cmin = C[i][j];
                ci = i;
                cj = j;
            }
        }
    }

    // println!("{}, {}, {}", cmin, ci, cj);
    let mut A: Vec<i32> = vec![0; N];
    let mut B: Vec<i32> = vec![0; N];

    A[ci] = 0;
    B[cj] = cmin;

    for i in 0..N {
        A[i] = C[i][cj] - cmin;
        // println!("{}, {}, {}", A[i], i, cj);
        if A[i] < 0 {
            println!("No");
            return;
        }
    }
    // println!("{:?}", A);
    let mut pre_b: Vec<i32> = vec![0; N];
    for i in 0..N {
        if i != 0 {
            pre_b = B.clone();
        }
        for j in 0..N {
            B[j] = C[i][j] - A[i];
            if B[j] < 0 {
                println!("No");
                return;
            }
        }
        if i != 0 {
            for j in 0..N {
                if B[j] != pre_b[j] {
                    println!("No");
                    return;
                }
            }
        }
    }
    
    println!("Yes");
    let pa: Vec<String> = A.iter().map(|x| x.to_string()).collect();
    println!("{}", pa.join(" "));
    let pb: Vec<String> = B.iter().map(|x| x.to_string()).collect();
    println!("{}", pb.join(" "));
    // for i in 0..N {
    //     print!("{} ", A[i]);
    // }
    // println!("");
    // for j in 0..N {
    //     print!("{} ", B[j]);
    // }
    // println!("");
    
}
