#[cfg(test)]
mod number_of_subsequences_tests {
    use competitive_programming::dp::number_of_subsequences;

    #[test]
    fn empty_subsequence_test() {
        let sequence: Vec<&str> = Vec::new();
        let modulo = 998244353;
        let expected = 0;

        let count = number_of_subsequences(&sequence, modulo);

        assert_eq!(count, expected);
    }

    #[test]
    fn zeroes_and_ones_test() {
        let sequence = [0, 0, 0, 1, 1];
        let modulo = 998244353;
        let expected = 11;

        let count = number_of_subsequences(&sequence, modulo);

        assert_eq!(count, expected);
    }

    #[test]
    fn sample_from_task_test() {
        let sequence = vec![9, 9, 8, 2, 4, 4, 3, 5, 3];
        let modulo = 998244353;

        let expected = 251;

        let count = number_of_subsequences(&sequence, modulo);

        assert_eq!(count, expected);
    }
}
