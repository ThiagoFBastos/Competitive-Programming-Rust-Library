/**
 * Sum integers inside a given interval
 * @param a the leftmost number inside the interval
 * @param b the rightmost number inside the interval
 * @return the sum of all integers between a and b
 */
fn linsum(a: i128, b: i128) -> i128 {
    (a + b) * (b - a + 1) / 2
}

/**
 * Find the \sum_{i=0}^{n-1} \left( (a i + b) \bmod m \right)
 */
fn sum(n: i128, m: i128, a: i128, b: i128) -> i128 {
    if a == 0 {
        return b * n;
    }

    let k = (a * (n - 1) + b) / m;
    let s = b * n + a * linsum(0, n - 1);
    let t = k * (n - 1)
        - ((a + (-b - 1) % a) % a + m * linsum(1, k)
            - k * (b + 1)
            - sum(k + 1, a, m % a, (a + (-b - 1) % a) % a))
            / a;

    s - m * t
}

/**
 * Find \sum_{i=0}^{n-1} \left\lfloor \frac{a i + b}{m} \right\rfloor
 */
pub fn sum_floor(n: i128, m: i128, a: i128, b: i128) -> i128 {
    let s = b * n + a * linsum(0, n - 1) - sum(n, m, a, b);
    s / m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        assert_eq!(sum_floor(4, 10, 6, 3), 3);
        assert_eq!(sum_floor(6, 5, 4, 3), 13);
        assert_eq!(sum_floor(1, 1, 0, 0), 0);
        assert_eq!(sum_floor(31415, 92653, 58979, 32384), 314095480);
        assert_eq!(
            sum_floor(1000000000, 1000000000, 999999999, 999999999),
            499999999500000000
        );
    }
}
