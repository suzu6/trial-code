fn main() {
  proconio::input! {
      n: i32,
  }
  println!("{}", century(n))
}

fn century(n: i32)-> i32{
    return (n + 100 - 1) / 100
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn century_test() {
        assert_eq!(super::century(1), 1);
        assert_eq!(super::century(99), 1);
        assert_eq!(super::century(100), 1);
        assert_eq!(super::century(101), 2);
        assert_eq!(super::century(199), 2);
        assert_eq!(super::century(200), 2);
        assert_eq!(super::century(201), 3);
        assert_eq!(super::century(2000), 20);
        assert_eq!(super::century(2001), 21);
    }
}