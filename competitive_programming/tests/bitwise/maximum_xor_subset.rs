#[cfg(test)]
mod maximum_xor_subset_tests {
    use competitive_programming::bitwise::maximum_xor_subset;

    #[test]
    fn sample_from_task_test() {
        assert_eq!(maximum_xor_subset(&[1, 6, 12, 6]), 13);
        assert_eq!(
            maximum_xor_subset(&[48, 33, 96, 77, 67, 59, 35, 15, 14, 86]),
            127
        );
        assert_eq!(maximum_xor_subset(&[5, 1, 2]), 7);
        assert_eq!(maximum_xor_subset(&[7, 8, 1, 2, 16, 32]), 63);
    }
}
