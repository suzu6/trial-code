use num_traits::PrimInt;

/// 1からnまでの総和を算出する
pub fn sum_one_to_n<T: PrimInt>(n: T) -> i32 {
  let n: i32 = n.to_i32().unwrap();
  if n <= 0 {
    panic!("n is not natural number.");
  }
  if n == 1 {
    1
  } else {
    n + sum_one_to_n(n - 1)
  }
}

/// 最大公約数
pub fn gcd(a: i32, b: i32) -> i32 {
  if a < 0 || b < 0 {
    panic!("a or b is not natural number.");
  }
  gcd_recrusive(max(a, b), min(a, b))
}
/// ユークリッドの互除法
fn gcd_recrusive(a: i32, b: i32) -> i32 {
  let c = a % b;
  if c == 0 {
    b
  } else {
    gcd_recrusive(min(a, b), c)
  }
}

/// 小さい方を返す
pub fn min(a: i32, b: i32) -> i32 {
  if a < b {
    a
  } else {
    b
  }
}

/// 大きい方を返す
pub fn max(a: i32, b: i32) -> i32 {
  if a > b {
    a
  } else {
    b
  }
}
