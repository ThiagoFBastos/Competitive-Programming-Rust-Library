#[cfg(test)]
mod edit_distance_tests {
    use competitive_programming::dp::edit_distance;

    #[test]
    fn simple_test() {
        let s = String::from("LOVE");
        let t = String::from("MOVIE");

        let expected = 2_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn without_operations_test() {
        let s = String::from("A");
        let t = String::from("A");

        let expected = 0_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn only_one_operation_test() {
        let s = String::from("A");
        let t = String::from("B");

        let expected = 1_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn many_operations_test() {
        let s = String::from("TWXFUABGBNLTBFNSUVQW");
        let t = String::from("GPNJILFXJUIZPLTVUIB");

        let expected = 19_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn many_replacements_test() {
        const LENGTH: usize = 300;

        let s = (0..LENGTH).map(|_| 'A').collect::<String>();
        let t = (0..LENGTH).map(|_| 'B').collect::<String>();

        let expected = LENGTH;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }
}
