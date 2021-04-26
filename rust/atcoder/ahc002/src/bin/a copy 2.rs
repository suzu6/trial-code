use std::time::Instant;

const TIMER: u128 = 1800;

fn main() {
    proconio::input! {
        s: (usize, usize),
        t_map: [[usize; 50]; 50],
        p_map: [[usize; 50]; 50],
    }
    let start: Instant = Instant::now();


    let seen: Vec<Vec<bool>> = vec![vec![false; 50]; 50]; 
    
    let empty: Vec<char> = vec![]; 
    let dir: Vec<char> = vec!['U', 'L', 'R', 'D']; 
    
    let result: (Vec<char>, usize) = f(
        s,
        &dir,
        &seen.clone(),
        &t_map,
        &p_map,
        &empty,
        start,
        0,
    );
    
    let ans: Vec<String> = result.0.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(""));
    println!("{} {}", result.0.len(), result.1);
}


fn f(
    s: (usize, usize), 
    dir: &Vec<char>,
    _seen: &Vec<Vec<bool>>,
    t_map: &Vec<Vec<usize>>,
    p_map: &Vec<Vec<usize>>,
    path_txt: &Vec<char>,
    start: Instant,
    p_total: usize,
) -> (Vec<char>, usize)  {
    let end = start.elapsed();
    
    if end.as_millis() > TIMER {
        return (path_txt.clone(), p_total);
    }
    let mut seen = _seen.clone();
    seen[s.0][s.1] = true;

    for d in dir {
        if is_outrange(s, *d) {
            continue;
        }
        let p = set_point(s, *d);
        if is_same_tile(s, p, t_map) {
            seen[p.0][p.1] = true;
            break;
        }
    }
    let mut max: (Vec<char>, usize) = (vec![], 0);

    for d in dir {
        if is_outrange(s, *d) {
            continue;
        }
        let p = set_point(s, *d);
        if seen[p.0][p.1] {
            continue;
        }
        if is_same_tile(s, p, t_map) {
            seen[p.0][p.1] = true;
            continue;
        }
        let mut pt = path_txt.clone();
        pt.push(*d);
        let res = f(
            p,
            &dir,
            &seen.clone(),
            &t_map,
            &p_map,
            &pt,
            start,
            p_total + p_map[p.0][p.1],
        );
        // ポイントが大きい方を採用
        if max.1 < res.1 {
            max = res;
        }
        // 時間を超えたら返す。
        let end = start.elapsed();
        if end.as_millis() > TIMER {
            break;
        }
    }

    return max;
}

/// 次の方向のポイント
fn set_point(s:(usize, usize), d: char) -> (usize, usize){
    match d {
        'U' => (s.0-1, s.1),
        'D' => (s.0+1, s.1),
        'L' => (s.0, s.1-1),
        'R' => (s.0, s.1+1),
        _ => (s.0, s.1),
    }
}

/// アウトレンジ
fn is_outrange(
    s: (usize, usize),
    d: char,
) -> bool {
    return match d {
        'U' => s.0 == 0,
        'D' => s.0 == 49,
        'L' => s.1 == 0,
        'R' => s.1 == 49,
        _ => false,
    }
}



// 同じタイルか
fn is_same_tile(
    s: (usize, usize), 
    p: (usize, usize), 
    t_map: &Vec<Vec<usize>>
) -> bool {
    return t_map[p.0][p.1] == t_map[s.0][s.1];
}
