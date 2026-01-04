
#[cfg(test)]
mod kmp_tests {

    use competitive_programming::string::kmp::kmp;

    #[test]
    fn test_simple_match() {
        
        let s = String::from("abc");
        let t = String::from("xyzabc");

        let pattern = format!("{}#{}", s, t);

        let suf = kmp(&pattern);

        assert_eq!(*suf.iter().max().unwrap(), 3);
    }

    #[test]
    fn test_multiple_pattern_match() {
        let s = String::from("abc");
        let t = String::from("abcxabcxabcxabc");

        let pattern = format!("{}#{}", s, t);

        let suf = kmp(&pattern);

        let count = suf.iter().filter(|element| **element == s.len()).count();

        assert_eq!(count, 4);
        assert_eq!(*suf.iter().max().unwrap(), 3);
    }

    #[test]
    fn test_simple_pattern_match_without_any_match() {
        let s = String::from("abc");
        let t = String::from("cba");

        let pattern = format!("{}#{}", s, t);

        let suf = kmp(&pattern);

        let count = suf.iter().filter(|element| **element == s.len()).count();

        assert_eq!(count, 0);
        assert_eq!(*suf.iter().max().unwrap(), 1);
    }
}
