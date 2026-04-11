/**
 * @brief Count the number of one bits in the binary representations of integers between 1 and n
 */
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
