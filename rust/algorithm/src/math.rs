/// 1からnまでの総和を算出する
pub fn sum_one_to_n(n: i32) -> i32 {
  if n <= 0 {
    panic!("n is not natural number.");
  }
  if n == 1 {
    1
  }else{
    n + sum_one_to_n(n - 1)
  }
}
