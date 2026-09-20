/// Computes the total number of set bits (`1`s) across the binary
/// representations of every integer in the range `[0, n]`.
///
/// In other words, this returns `Σ popcount(k)` for `k` from `0` to `n`
/// inclusive (equivalently `1` to `n`, since `popcount(0) == 0`).
///
/// Note: despite the name mirroring the classic "Counting Bits" problem,
/// this function does **not** return a per-number breakdown — it returns
/// the single aggregate total.
///
/// # Algorithm
///
/// A naive solution would iterate every integer from `0` to `n` and sum
/// `count_ones()`, which is `O(n)`. This function instead computes the
/// result in `O(log n)` by processing the bits of `n` from least
/// significant to most significant, and relying on a well-known identity:
///
/// > Among any `2^i` consecutive integers starting at a multiple of `2^i`,
/// > each of the `i` low bit positions is set in exactly half of them, so
/// > the total number of set bits across a full block of `2^i` integers
/// > is `i * 2^(i-1)`.
///
/// At each iteration `i`, the function tracks:
/// - `power_of_two`, equal to `2^i`.
/// - `remainder`, equal to `1 + (original n mod 2^i)` — i.e. one more than
///   the value formed by the lowest `i` bits of `n` processed so far.
/// - `i`, the current bit index.
///
/// Whenever bit `i` of the (shrinking) `n` is `1`, two quantities are
/// folded into the running total `ones`:
/// 1. `i * (power_of_two >> 1)` — the set-bit total contributed by a full
///    block of `2^i` integers, as described above.
/// 2. `remainder` — the extra set bits contributed by bit `i` itself being
///    "on" across the partial block of `remainder` numbers defined by the
///    lower bits of `n` already processed.
///
/// After a bit is processed, `n` is shifted right and `power_of_two`,
/// `remainder`, and `i` are advanced, so the loop naturally terminates
/// once all significant bits of the original `n` have been consumed.
///
/// # Arguments
///
/// * `n` - A non-negative integer defining the inclusive upper bound of
///   the range `[0, n]` to sum bit counts over. Taken by value; the
///   parameter is mutated internally purely as scratch state and this has
///   no effect on the caller.
///
/// # Returns
///
/// The sum of `count_ones()` for every integer from `0` to `n` inclusive.
/// Returns `0` if `n <= 0` (the loop never executes), so negative inputs
/// are effectively treated as an empty range rather than producing a
/// mathematically meaningful result — callers should only pass
/// non-negative values.
///
/// # Complexity
///
/// Time: `O(log n)`, bounded by the number of bits in `n` (at most 63
/// iterations for a positive `i64`). Space: `O(1)`.
pub fn counting_bits(mut n: i64) -> i64 {
    let mut ones = 0;
    let mut remainder = 1;
    let mut power_of_two = 1;
    let mut i = 0;

    while n > 0 {
        ones += (n & 1) * (remainder + (power_of_two >> 1) * i);
        remainder += (n & 1) * power_of_two;
        power_of_two <<= 1;
        n >>= 1;
        i += 1;
    }

    ones
}
