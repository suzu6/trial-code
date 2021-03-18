pub mod math;
extern crate num_traits;

#[cfg(test)]
mod tests {

    mod test_sum{
        #[test]
        fn test_sum() {
            assert_eq!(
                crate::math::sum_one_to_n(10),
                55
            );
        }
        
        #[test]
        fn test_sum_one() {
            assert_eq!(
                crate::math::sum_one_to_n(1),
                1
            );
        }
    
        #[test]
        #[should_panic]
        fn test_sum_minus() {
            let minus: i32 = -1;
            // 負の値はpanicを返す
            crate::math::sum_one_to_n(minus);
        }
    
        #[test]
        #[should_panic]
        fn test_sum_zero() {
            let zero: i32 = 0;
            // 0はpanicを返す
            crate::math::sum_one_to_n(zero);
        }
    }

    mod test_gcd{
        #[test]
        fn test_gcd() {
            assert_eq!(12, crate::math::gcd(12, 24));
            assert_eq!(3, crate::math::gcd(12, 21));
            assert_eq!(1, crate::math::gcd(12, 1));
        }
    }
}