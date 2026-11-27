/**
 * @brief binary exponentiation
 * @param n the number to calculate the power
 * @param p the exponent
 * @param m the modulo
 * @return n^p mod m
 */
fn bin_exp(n: i128, p: i128, m: i128) -> i128 {
    if p == 0 {
        return 1;
    }

    let mut res = bin_exp(n, p >> 1, m);

    res = res * res % m;

    if p & 1 == 1 {
        res = res * n % m;
    }

    res
}

/**
 * Miller-Rabin algorithm to check primes
 * @param n the number to check if is prime or not
 * @return a boolean that is true if n is prime and false otherwise
 */
pub fn is_prime(n: i64) -> bool {
    let primes: [i64; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

    for &p in primes.iter() {
        if n % p == 0 {
            return n == p;
        }
    }

    if n < *primes.last().unwrap() {
        return false;
    }

    let mut t = n - 1;
    let mut s = 0;

    while t & 1 == 0 {
        t >>= 1;
        s += 1;
    }

    for &p in primes.iter() {
        let mut pow = bin_exp(p as i128, t as i128, n as i128);

        if pow == 1 {
            continue;
        }

        let mut ok = false;

        for _ in 0..s {
            if pow == n as i128 - 1 {
                ok = true;
                break;
            }

            pow = pow * pow % n as i128;
        }

        if !ok {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_numbers() {
        // pequenos
        assert!(is_prime(29));
        assert!(is_prime(101));
        assert!(is_prime(10_007));

        // médios
        assert!(is_prime(524_287));
        assert!(is_prime(6_700_417));
        assert!(is_prime(1_000_003));

        // grandes (bem conhecidos)
        assert!(is_prime(1_000_000_007));
        assert!(is_prime(1_000_000_009));
        assert!(is_prime(9_999_999_967i64));

        // muito grande (~5.7e17)
        assert!(is_prime(723_440_386_903_574_573i64));
        assert!(is_prime(76_977_702_294_133_579i64));
        assert!(is_prime(362_977_551_925_746_343i64));
    }

    #[test]
    fn test_composite_numbers() {
        // pequenos
        assert!(!is_prime(25));
        assert!(!is_prime(91));
        assert!(!is_prime(10_000));

        // médios
        assert!(!is_prime(524_288));
        assert!(!is_prime(1_000_001));
        assert!(!is_prime(6_700_416));

        // grandes
        assert!(!is_prime(1_000_000_008));
        assert!(!is_prime(1_000_000_014));
        assert!(!is_prime(9_999_999_966));

        // muito grande (~5.7e17)
        assert!(!is_prime(576_460_752_303_423_488i64));
    }
}
