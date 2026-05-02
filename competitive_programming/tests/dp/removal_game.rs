#[cfg(test)]
mod removal_game_tests {
    use competitive_programming::dp::removal_game;

    #[test]
    fn sample_from_task_test() {
        assert_eq!(removal_game(&[-5, 5, 9, -4, 10, -9, 0, 3, 2, -6]), 16);
        assert_eq!(removal_game(&[0, -3, -9, -9, -3, 5, 6, 3, 8, 5]), 2);
        assert_eq!(removal_game(&[-5, 5, -5, 6, -8, -9, -6, 3, 0, 2]), 7);
        assert_eq!(removal_game(&[-7, 6, -8, -7, -10, -8, -7, 10, -6, -10]), -9);
        assert_eq!(removal_game(&[-8, 4, 6, -2, 5, -4, -5, 9, 10, 1]), 13);
        assert_eq!(removal_game(&[1, 2, 9]), 10);
        assert_eq!(removal_game(&[5]), 5);
    }
}
