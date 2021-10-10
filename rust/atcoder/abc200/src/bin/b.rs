fn main() {
  proconio::input! {
      n: i64,
      k: i64,
  }
  println!("{}", solution(n, k))
}

fn solution(n: i64, k: i64)-> i64{
    let mut result = n;
    for _ in 0..k {
        result = div200(result)
    }
    return result
}

fn set200(n: i64)-> i64{
    return n * 1000 + 200
}

fn div200(n: i64)-> i64{
    if (n % 200 == 0) {
        return n/200
    } else {
        return set200(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn set200_test() {
        assert_eq!(super::set200(1), 1200);
        assert_eq!(super::set200(7), 7200);
        assert_eq!(super::set200(10000001), 10000001200);
    }

    #[test]
    fn div200_test() {
        assert_eq!(super::div200(1), 1200);
        assert_eq!(super::div200(7), 7200);
        assert_eq!(super::div200(200), 1);
        assert_eq!(super::div200(400), 2);
        assert_eq!(super::div200(1200), 6);
        assert_eq!(super::div200(10000001), 10000001200);
    }
    #[test]
    fn solution_test() {
        assert_eq!(super::solution(2021, 4), 50531);
        // assert_eq!(super::solution(4000, 2), 1);
        // assert_eq!(super::solution(8691, 20), 84875488281);
    }
}