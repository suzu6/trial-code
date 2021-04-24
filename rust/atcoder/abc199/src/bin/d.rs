

fn main() {
  proconio::input! {
      n: usize,
      m: usize,
      ab: [(usize, usize); m],
  }


  let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

  for i in 0..m {
    graph[ab[i].0-1].push(ab[i].1-1);
    graph[ab[i].1-1].push(ab[i].0-1);
  }

  for i in 0..n {
      if graph[i].len() > 2{
          println!("0");
          return;
      }
  }

  let mut memo = vec![100; n];
  memo[0] = 0;
  let ans = f(&mut graph, 1, n, memo);

  println!("{}", ans*3);
}

fn f(graph: &mut Vec<Vec<usize>>, index: usize, n:usize, memo:Vec<usize>) -> usize{
    if index == n {
        return 1;
    }
    if memo[index] != 100{
        return 0;
    }

    let mut count = 0;
    for i in 0..3{
        let mut flag = true;
        let gn = graph[index].len();
        for j in 0..gn {
            if i == memo[j] {
                flag = false;
            }
        }
        if flag {
            let mut m = memo.clone();
            m[index] = i;
            count += f(graph, index+1, n, m);
        }
    }

    return count;
}