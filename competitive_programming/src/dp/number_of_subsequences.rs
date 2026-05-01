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
