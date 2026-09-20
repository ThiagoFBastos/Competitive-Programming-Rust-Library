/// Computes the maximum XOR value obtainable from any subset of `arr`.
///
/// This finds the largest value achievable by XOR-combining zero or more
/// elements of `arr`. It works by building a **linear basis (XOR basis)**
/// of the input values using Gaussian elimination over GF(2), then
/// greedily combining basis vectors to maximize the result.
///
/// # Algorithm
///
/// The function processes bits from the most significant (bit 63) down to
/// the least significant (bit 0). At each bit position `i`:
///
/// 1. It scans the current working set of values (`values`) and picks the
///    largest one that has bit `i` set — this becomes a basis vector for
///    that bit position (equivalent to Gaussian elimination's pivot step).
/// 2. If `result` can be increased by XOR-ing it with this basis vector
///    (i.e. `result ^ max_element > result`), the basis vector is folded
///    into `result`. Because bit `i` is processed from high to low, this
///    greedy choice is always optimal — a higher set bit always dominates
///    any combination of lower bits.
/// 3. Every other value in the working set that also has bit `i` set is
///    XOR-ed with the chosen basis vector, clearing bit `i` from it. This
///    is the elimination step: it ensures each bit position is "owned" by
///    at most one basis vector, so later iterations don't reuse the same
///    information.
///
/// After all 64 bit positions have been processed, `result` holds the
/// maximum XOR value obtainable from any subset of `arr`.
///
/// # Arguments
///
/// * `arr` - A slice of unsigned 64-bit integers. Order does not matter;
///   the empty subset (contributing 0) is implicitly considered.
///
/// # Returns
///
/// The maximum XOR value achievable by XOR-ing together any subset
/// (including the empty subset, which yields `0`) of the elements in `arr`.
/// Returns `0` if `arr` is empty or contains only zeros.
///
/// # Complexity
///
/// Time: `O(64 * n)`, where `n = arr.len()`, since each of the 64 bit
/// positions requires a full scan (and possible update) of the working
/// values. Space: `O(n)` for the cloned working vector.
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
