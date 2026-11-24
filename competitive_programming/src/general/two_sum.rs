use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Sub;

pub fn two_sum<T: Copy + Clone + Sub<Output = T> + Hash + Eq>(
    values: &[T],
    target_sum: T,
) -> Option<(usize, usize)> {
    let mut position = HashMap::new();

    for (i, value) in values.iter().enumerate() {
        let target = target_sum - *value;

        if let Some(pos) = position.get(&target) {
            return Some((*pos, i));
        }

        position.insert(value, i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_pair_with_target_sum_test() {
        const SUM: i32 = 12;

        let values = [6, 1, 2, 3, 4, 5, 6];

        let result = two_sum(&values, SUM);

        assert_ne!(result, None);

        let (i, j) = result.unwrap();

        assert!(i.max(j) < values.len());

        let found_sum = values[i] + values[j];

        assert_ne!(i, j);
        assert_eq!(found_sum, SUM);
    }

    #[test]
    fn empty_array_test() {
        let values: Vec<i32> = vec![];

        let result = two_sum(&values, 0);

        assert_eq!(result, None);
    }

    #[test]
    fn single_value_doesnt_has_sum_test() {
        const SUM: i32 = 12;

        let values = [1, 2, 3, 4, 5, 6];

        let result = two_sum(&values, SUM);

        assert_eq!(result, None);
    }
}
