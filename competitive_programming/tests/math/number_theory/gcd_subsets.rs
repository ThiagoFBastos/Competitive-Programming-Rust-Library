

#[cfg(test)]
mod gcd_subsets_tests {
    use competitive_programming::math::number_theory::gcd_subsets::count_gcd_subsets;


    #[test]
    fn test_case_1() {
        const MOD: i64 = 1_000_000_007;

        let values = vec![5, 4, 4, 2, 3];
        let expected = vec![22, 4, 1, 3, 1];

        let result = count_gcd_subsets::<MOD>(&values);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        const MOD: i64 = 1_000_000_007;

        let values = vec![8, 8, 1, 1, 1, 2, 7, 7, 8, 4];
        let expected = vec![989, 16, 0, 8, 0, 0, 3, 7, 0, 0];

        let result = count_gcd_subsets::<MOD>(&values);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        const MOD: i64 = 1_000_000_007;

        let values = vec![1, 1, 1];
        let expected = vec![7, 0, 0];

        let result = count_gcd_subsets::<MOD>(&values);

        assert_eq!(result, expected);
    }
}