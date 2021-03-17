pub mod math;

#[cfg(test)]
mod tests {
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
        // 負の値はpanicを返す
        crate::math::sum_one_to_n(-1);
    }

    #[test]
    #[should_panic]
    fn test_sum_zero() {
        // 0はpanicを返す
        crate::math::sum_one_to_n(0);
    }
}