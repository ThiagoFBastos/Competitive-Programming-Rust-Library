use std::{collections::HashMap, hash::Hash};

/**
 * @brief Count the number of distinct subsequences
 */
pub fn number_of_subsequences<T: Copy + Clone + Eq + Hash>(sequence: &[T], modulo: i32) -> i32 {
    let n = sequence.len();
    let mut dp = HashMap::new();
    let mut result = 0;

    dp.reserve(n);

    for &value in sequence.iter() {
        let number_subsequences = *dp.get(&value).unwrap_or(&0);
        let count = (1 + result - number_subsequences) % modulo;

        result = (result + count) % modulo;

        dp.insert(value, (number_subsequences + count) % modulo);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
