pub mod math;
extern crate num_traits;

#[cfg(test)]
mod tests {

    mod test_sum {
        #[test]
        fn test_sum() {
            assert_eq!(crate::math::sum_one_to_n(10), 55);
        }
        #[test]
        fn test_sum_one() {
            assert_eq!(crate::math::sum_one_to_n(1), 1);
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

    mod test_gcd {
        #[test]
        fn test_gcd() {
            assert_eq!(12, crate::math::gcd(12, 24));
            assert_eq!(3, crate::math::gcd(12, 21));
            assert_eq!(1, crate::math::gcd(12, 1));
        }
        #[test]
        #[should_panic]
        fn test_arg_zero() {
            crate::math::gcd(12, 0);
        }
        #[test]
        #[should_panic]
        fn test_arg_minus() {
            crate::math::gcd(-2, -3);
        }
    }

    mod test_lcm {
        #[test]
        fn test_lcm() {
            assert_eq!(24, crate::math::lcm(12, 24));
            assert_eq!(84, crate::math::lcm(12, 21));
            assert_eq!(12, crate::math::lcm(12, 1));
        }
        #[test]
        #[should_panic]
        fn test_arg_zero() {
            crate::math::lcm(12, 0);
        }
        #[test]
        #[should_panic]
        fn test_arg_minus() {
            crate::math::lcm(-2, -3);
        }
    }

    
    mod test_fractal {
        #[test]
        fn test_fractal() {
            assert_eq!(720, crate::math::factorial(6));
        }
        #[test]
        fn test_arg_zero() {
            assert_eq!(1, crate::math::factorial(0));
        }
        #[test]
        #[should_panic]
        fn test_arg_minus() {
            crate::math::factorial(-2);
        }
    }

    mod test_pow {
        #[test]
        fn test_pow() {
            assert_eq!(4, crate::math::pow(2, 2));
        }
        #[test]
        fn test_arg_zero() {
            assert_eq!(1, crate::math::pow(3, 0));
        }
        #[test]
        fn test_arg_minus() {
            assert_eq!(4, crate::math::pow(-2, 2));
        }
    }
}
