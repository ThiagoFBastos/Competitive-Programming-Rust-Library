#[cfg(test)]
mod scanner_tests {
    use competitive_programming::io::Scanner;
    use std::io::{self, Cursor};

    #[test]
    fn test_input_read() {
        let input = Cursor::new("10 20 hello");

        let mut sc = Scanner::new(input);

        assert!(sc.has_next());
        assert_eq!(sc.next::<i32>().unwrap(), 10);
        assert!(sc.has_next());
        assert_eq!(sc.next::<i32>().unwrap(), 20);
        assert!(sc.has_next());
        assert_eq!(sc.next::<String>().unwrap(), "hello".to_string());
        assert!(!sc.has_next());
    }

    #[test]
    fn test_eof() {
        let input = Cursor::new("");

        let mut sc = Scanner::new(input);

        assert!(!sc.has_next());

        let res = sc.next::<usize>();

        assert!(res.is_err());

        let err = res.err().unwrap();

        assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn test_invalid_input() {
        let input = Cursor::new("hello");

        let mut sc = Scanner::new(input);

        assert!(sc.has_next());

        let res = sc.next::<usize>();

        assert!(res.is_err());

        let err = res.err().unwrap();

        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
