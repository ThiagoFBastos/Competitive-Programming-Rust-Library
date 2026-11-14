use std::{
    io::{self, Error, Read},
    marker::PhantomData,
};

pub struct Scanner<R> {
    buffer: Vec<u8>,
    index: usize,
    reader: PhantomData<R>,
}

impl<R: Read> Scanner<R> {
    /**
     * @brief Create a new instance of Scanner type
     */
    pub fn new(mut reader: R) -> Self {
        let mut input = Vec::new();

        reader.read_to_end(&mut input).unwrap();

        Self {
            buffer: input,
            index: 0,
            reader: PhantomData,
        }
    }

    /**
     * @brief Check if the input has data to be read
     */
    pub fn has_next(&mut self) -> bool {
        while self.index < self.buffer.len() && self.buffer[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        self.index < self.buffer.len()
    }

    /**
     * @brief Read the next data from input
     */
    #[allow(clippy::should_implement_trait)]
    pub fn next<T: std::str::FromStr>(&mut self) -> io::Result<T> {
        if !self.has_next() {
            return Err(Error::new(io::ErrorKind::UnexpectedEof, "End Of File"));
        }

        let start = self.index;

        while self.index < self.buffer.len() && !self.buffer[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        let res = std::str::from_utf8(&self.buffer[start..self.index])
            .unwrap()
            .parse::<T>();

        if res.is_ok() {
            return Ok(res.ok().unwrap());
        }

        Err(Error::new(io::ErrorKind::InvalidInput, "Invalid Input"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
