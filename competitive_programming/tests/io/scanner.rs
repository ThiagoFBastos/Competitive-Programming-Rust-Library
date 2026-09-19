#[cfg(test)]
mod scanner_tests {
    use competitive_programming::io::Scanner;
    use std::{assert_eq, io::Cursor};

    #[test]
    fn test_input_read() {
        let input = Cursor::new("10 20 hello");

        let mut sc = Scanner::new(input);

        assert!(sc.has_next());
        assert_eq!(sc.next::<i32>(), 10);
        assert!(sc.has_next());
        assert_eq!(sc.next::<i32>(), 20);
        assert!(sc.has_next());
        assert_eq!(sc.next::<String>(), "hello".to_string());
    }

    #[test]
    #[should_panic]
    fn test_eof() {
        let input = Cursor::new("");

        let mut sc = Scanner::new(input);

        _ = sc.next::<usize>();
    }
}
