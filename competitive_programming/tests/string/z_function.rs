
#[cfg(test)]
mod z_function_tests {
    use competitive_programming::string::z_function::*;

    #[test]
    fn test_yosupo_sample_test_case_1() {
        let str = String::from("abcbcba");

        let z = z_function(&str);

        let expected = vec![0, 0, 0, 0, 0, 0, 1];

        assert_eq!(z, expected);
    }

    #[test]
    fn test_yosupo_sample_test_case_2() {
        let str = String::from("mississippi");

        let z = z_function(&str);

        let expected = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(z, expected);
    }

    #[test]
    fn test_yosupo_sample_test_case_3() {
        let str = String::from("ababacaca");

        let z = z_function(&str);

        let expected = vec![0, 0, 3, 0, 1, 0, 1, 0, 1];

        assert_eq!(z, expected);
    }

    #[test]
    fn test_yosupo_sample_test_case_4() {
        let str = String::from("aaaaa");

        let z = z_function(&str);

        let expected = vec![0, 4, 3, 2, 1];

        assert_eq!(z, expected);
    }
}
