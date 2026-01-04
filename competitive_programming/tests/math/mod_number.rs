
#[cfg(test)]
mod mod_number_tests {
    use competitive_programming::math::mod_number::ModNumber;

    #[test]
    fn test_constructor_with_negative_one() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(-1);

        assert_eq!(number.value(), MOD - 1);
    }

    #[test]
    fn test_constructor_with_small_negative() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(-MOD);

        assert_eq!(number.value(), 0);
    }

     #[test]
    fn test_constructor_with_large_negative() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(-2 * MOD - 1);

        assert_eq!(number.value(), MOD - 1);
    }

    #[test]
    fn test_constructor_with_small_positive() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(2 * MOD - 1);

        assert_eq!(number.value(), MOD - 1);
    }

    #[test]
    fn test_constructor_with_large_positive() {
        const MOD: i64 = 1_000_000_007;

        let number = ModNumber::<MOD>::new(2 * MOD);

        assert_eq!(number.value(), 0);
    }

    #[test]
    fn test_add_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(MOD - 1);
        let num2 = ModNumber::<MOD>::new(MOD - 1);

        let result = num1 + num2;

        assert_eq!(result.value(), MOD - 2);
    }

    #[test]
    fn test_subtract_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(0);
        let num2 = ModNumber::<MOD>::new(1);

        let result = num1 - num2;

        assert_eq!(result.value(), MOD - 1);
    }

    #[test]
    fn test_multiply_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(191919181);
        let num2 = ModNumber::<MOD>::new(81837373);

        let result = num1 * num2;

        assert_eq!(result.value(), 491_408_386);
    }

    #[test]
    fn test_divide_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(4);
        let num2 = ModNumber::<MOD>::new(2);

        let result = num1 / num2;

        assert_eq!(result.value(), 2);
    }

    #[test]
    fn test_add_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(MOD - 1);
        result += ModNumber::<MOD>::new(MOD - 1);

        assert_eq!(result.value(), MOD - 2);
    }

    #[test]
    fn test_subtract_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(0);
        result -= ModNumber::<MOD>::new(1);

        assert_eq!(result.value(), MOD - 1);
    }

    #[test]
    fn test_multiply_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(191919181);
        result *= ModNumber::<MOD>::new(81837373);

        assert_eq!(result.value(), 491_408_386);
    }

    #[test]
    fn test_divide_assign_numbers() {
        const MOD: i64 = 1_000_000_007;

        let mut result = ModNumber::<MOD>::new(4);
        result /= ModNumber::<MOD>::new(2);

        assert_eq!(result.value(), 2);
    }

    #[test]
    fn test_raise_to_the_power() {
        const MOD: i64 = 1_000_000_007;

        let num = ModNumber::<MOD>::new(2);

        let result = num.pow(60);

        assert_eq!(result.value(), 536396504);
    }

    #[test]
    fn test_negative_the_number() {
        const MOD: i64 = 1_000_000_007;

        let num = ModNumber::<MOD>::new(1);

        let result = -num;

        assert_eq!(result.value(), MOD - 1);
    }

    #[test]
    fn test_multiply_large_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(939399393933939393);
        let num2 = ModNumber::<MOD>::new(848484292929928282);

        let result = num1 * num2;

        assert_eq!(result.value(), 258938338);
    }

    #[test]
    fn test_compare_equal_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(1);
        let num2 = ModNumber::<MOD>::new(MOD + 1);

        assert!(num1 == num2);
    }

    #[test]
    fn test_compare_different_numbers() {
        const MOD: i64 = 1_000_000_007;

        let num1 = ModNumber::<MOD>::new(1);
        let num2 = ModNumber::<MOD>::new(2);

        assert!(num1 != num2);
    }
}