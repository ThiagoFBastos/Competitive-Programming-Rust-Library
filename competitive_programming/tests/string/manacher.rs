
#[cfg(test)]
mod manacher_tests {
    use competitive_programming::string::manacher::*;

    #[test]
    fn test_yosupo_sample_test_case_1() {
        let str = String::from("abcbcba");

        let len = manacher(&str);

        assert_eq!(len, 7);
    }

    #[test]
    fn test_yosupo_sample_test_case_2() {
        let str = String::from("mississippi");

        let len = manacher(&str);

        assert_eq!(len, 7);
    }

    #[test]
    fn test_yosupo_sample_test_case_3() {
        let str = String::from("ababacaca");

        let len = manacher(&str);

        assert_eq!(len, 5);
    }

    #[test]
    fn test_yosupo_sample_test_case_4() {
        let str = String::from("aaaaa");

        let len = manacher(&str);

        assert_eq!(len, 5);
    }

    #[test]
    fn test_cses_sample_test_case_1() {
        let str = String::from("aybabtu");

        let len = manacher(&str);

        assert_eq!(len, 3);
    }

    #[test]
    fn test_cses_sample_test_case_2() {
        let str = String::from("ihpohpzoffel");

        let len = manacher(&str);

        assert_eq!(len, 2);
    }

    #[test]
    fn test_cses_sample_test_case_3() {
        let str = String::from("flexflexvpqxierullgcfckjqflexflex");

        let len = manacher(&str);

        assert_eq!(len, 3);
    }

    #[test]
    fn test_cses_sample_test_case_4() {
        let str = String::from("obsession");

        let len = manacher(&str);

        assert_eq!(len, 3);
    }

    #[test]
    fn test_cses_sample_test_case_5() {
        let str = String::from("abcxcbaxcba");

        let len = manacher(&str);

        assert_eq!(len, 7);
    }

    
    #[test]
    fn test_cses_sample_test_case_6() {
        let str = String::from("aaccaabbaaccaaccaabbaaccaa");

        let len = manacher(&str);

        assert_eq!(len, str.len());
    }

    #[test]
    fn test_cses_sample_test_case_7() {
        let str = String::from("a");

        let len = manacher(&str);

        assert_eq!(len, 1);
    }

    #[test]
    fn test_cses_sample_test_case_8() {
        let str = String::from("abcdba");

        let len = manacher(&str);

        assert_eq!(len, 1);
    }

    #[test]
    fn test_cses_sample_test_case_9() {
        let str = String::from("abb");

        let len = manacher(&str);

        assert_eq!(len, 2);
    }

    #[test]
    fn test_cses_sample_test_case_10() {
        let str = String::from("aaaaaaaaaa");

        let len = manacher(&str);

        assert_eq!(len, str.len());
    }    
}