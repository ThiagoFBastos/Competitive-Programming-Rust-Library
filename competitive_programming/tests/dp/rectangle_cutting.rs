#[cfg(test)]
mod rectangle_cutting_tests {
    use competitive_programming::dp::rectangle_cutting;

    #[test]
    fn sample_from_task_test() {
        assert_eq!(rectangle_cutting(2, 8), 3);
        assert_eq!(rectangle_cutting(4, 4), 0);
        assert_eq!(rectangle_cutting(1, 4), 3);
        assert_eq!(rectangle_cutting(5, 8), 4);
        assert_eq!(rectangle_cutting(5, 10), 1);
        assert_eq!(rectangle_cutting(404, 288), 10);
        assert_eq!(rectangle_cutting(349, 234), 13);
        assert_eq!(rectangle_cutting(2, 8), 3);
        assert_eq!(rectangle_cutting(180, 137), 12);
        assert_eq!(rectangle_cutting(201, 348), 10);
        assert_eq!(rectangle_cutting(132, 46), 9);
        assert_eq!(rectangle_cutting(1, 500), 499);
        assert_eq!(rectangle_cutting(1, 1), 0);
        assert_eq!(rectangle_cutting(500, 500), 0);
    }
}
