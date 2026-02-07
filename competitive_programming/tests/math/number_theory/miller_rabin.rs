

#[cfg(test)]
mod miller_rabin_tests {
    use competitive_programming::math::number_theory::miller_rabin::*;

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