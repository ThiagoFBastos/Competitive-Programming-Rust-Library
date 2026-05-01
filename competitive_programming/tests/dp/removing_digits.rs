#[cfg(test)]
mod removing_digits_tests {

    use competitive_programming::dp::removing_digits;

    #[test]
    fn sample_from_task_test() {
        assert_eq!(removing_digits(27), 5);
        assert_eq!(removing_digits(4), 1);
        assert_eq!(removing_digits(17), 3);
        assert_eq!(removing_digits(35), 7);
        assert_eq!(removing_digits(167), 29);
        assert_eq!(removing_digits(4434), 687);
        assert_eq!(removing_digits(9722), 1381);
        assert_eq!(removing_digits(37882), 5406);
        assert_eq!(removing_digits(103330), 13867);
        assert_eq!(removing_digits(562167), 75527);
        assert_eq!(removing_digits(991919), 127308);
        assert_eq!(removing_digits(999993), 128206);
        assert_eq!(removing_digits(999999), 128206);
        assert_eq!(removing_digits(23456), 3364);
        assert_eq!(removing_digits(1000000), 128207);
    }
}
