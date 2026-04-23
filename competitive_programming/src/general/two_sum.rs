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
