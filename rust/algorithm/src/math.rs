use num_traits::PrimInt;

/// 1からnまでの総和を算出する
pub fn sum_one_to_n<T: PrimInt>(n: T) -> u32 {
  let n: u32 = n.to_u32().unwrap();
  if n <= 0 {
    panic!("n is not natural number.");
  }
  if n == 1 {
    1
  }else{
    n + sum_one_to_n(n - 1)
  }
}
