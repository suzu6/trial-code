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

/// 最小公約数
pub fn lcm(a: i32, b: i32) -> i32 {
  if a < 1 || b < 1 {
    panic!("a or b is not natural number.");
  }
  (a * b) / gcd(a, b)
}

/// 最大公約数
pub fn gcd(a: i32, b: i32) -> i32 {
  if a < 1 || b < 1 {
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

/// 絶対値を返す
pub fn abs(x: i32) -> i32 {
  if x < 0 {
    -x
  } else {
    x
  }
}

// n!を返す
pub fn factorial(n: i32) -> i32 {
  if n < 0 {
    panic!("n is not integer.");
  }
  if n == 0 {
    return 1;
  }
  let mut result = n;
  for i in 1..n {
    result *= i;
  }
  result
}

// x^yを返す
pub fn pow(x: i32, y: u32) -> i32 {
  if y == 0 {
    return 1;
  }
  let mut dst = 1;
  for _ in 0..y {
    dst *= x; 
  }
  dst
}