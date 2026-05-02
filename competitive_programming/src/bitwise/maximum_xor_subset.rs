/**
 * @brief Find the value of a subset with the maximum xor value
 */
pub fn maximum_xor_subset(arr: &[u64]) -> u64 {
    const MAX_BITS: usize = 64;

    let mut result = 0;

    let mut values = arr.to_vec();

    for i in (0..MAX_BITS).rev() {
        let mut max_element = 0;

        for &value in values.iter() {
            if (value >> i) & 1 == 1 {
                max_element = max_element.max(value);
            }
        }

        if max_element == 0 {
            continue;
        }

        if (result ^ max_element) > result {
            result ^= max_element;
        }

        for value in values.iter_mut() {
            if (*value >> i) & 1 == 1 {
                *value ^= max_element;
            }
        }
    }

    result
}
