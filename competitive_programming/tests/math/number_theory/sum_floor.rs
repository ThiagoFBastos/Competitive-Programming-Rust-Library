#[cfg(test)]
mod tests {
    use competitive_programming::math::number_theory::sum_floor;

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
