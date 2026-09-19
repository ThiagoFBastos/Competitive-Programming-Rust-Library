#[cfg(test)]
mod convex_hull_trick_tests {
    use competitive_programming::data_structures::*;

    fn value_of(function: (i64, i64), x: i64) -> i64 {
        function.0 * x + function.1
    }

    fn brute_force_min(functions: &[(i64, i64)], x: i64) -> i64 {
        functions
            .iter()
            .map(|&f| value_of(f, x))
            .min()
            .unwrap_or(i64::MAX)
    }

    fn brute_force_max(functions: &[(i64, i64)], x: i64) -> i64 {
        functions
            .iter()
            .map(|&f| value_of(f, x))
            .max()
            .unwrap_or(i64::MIN)
    }

    #[test]
    fn empty_min_cht() {
        let mut cht = ConvexHullTrick::<false>::new();

        assert_eq!(cht.evaluate(0), i64::MAX);
        assert_eq!(cht.evaluate(100), i64::MAX);
    }

    #[test]
    fn empty_max_cht() {
        let mut cht = ConvexHullTrick::<true>::new();

        assert_eq!(cht.evaluate(0), i64::MIN);
        assert_eq!(cht.evaluate(100), i64::MIN);
    }

    #[test]
    fn single_function_min() {
        let mut cht = ConvexHullTrick::<false>::new();

        cht.add((2, 3));

        assert_eq!(cht.evaluate(-10), -17);
        assert_eq!(cht.evaluate(0), 3);
        assert_eq!(cht.evaluate(10), 23);
    }

    #[test]
    fn single_function_max() {
        let mut cht = ConvexHullTrick::<true>::new();

        cht.add((2, 3));

        assert_eq!(cht.evaluate(-10), -17);
        assert_eq!(cht.evaluate(0), 3);
        assert_eq!(cht.evaluate(10), 23);
    }

    #[test]
    fn min_cht_basic() {
        // Slopes: 3 > 2 > 1
        let functions = vec![(3, 0), (2, 5), (1, 10)];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -10..=10 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_basic() {
        // Slopes: 1 < 2 < 3
        let functions = vec![(1, 0), (2, 5), (3, 10)];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -10..=10 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn min_cht_removes_dominated_lines() {
        // Slopes: 3 > 2 > 1
        //
        // The middle line is never the minimum.
        let functions = vec![(3, 0), (2, 100), (1, 0)];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -100..=100 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_removes_dominated_lines() {
        // Slopes: 1 < 2 < 3
        //
        // The middle line is never the maximum.
        let functions = vec![(1, 0), (2, 100), (3, 0)];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -100..=100 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn min_cht_equal_slopes() {
        // Equal slopes are allowed, but the intercepts should
        // be handled correctly.
        let functions = vec![(2, 20), (2, 10), (2, 5)];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -10..=10 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_equal_slopes() {
        let functions = vec![(2, 5), (2, 10), (2, 20)];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -10..=10 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn min_cht_negative_slopes() {
        // Slopes: -1 > -2 > -3
        let functions = vec![(-1, 10), (-2, 0), (-3, -10)];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -20..=20 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_negative_slopes() {
        // Slopes: -3 < -2 < -1
        let functions = vec![(-3, -10), (-2, 0), (-1, 10)];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -20..=20 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn min_cht_negative_intercepts() {
        // Slopes: 3 > 2 > 1
        let functions = vec![(3, -10), (2, -50), (1, -100)];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -20..=20 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_negative_intercepts() {
        // Slopes: 1 < 2 < 3
        let functions = vec![(1, -100), (2, -50), (3, -10)];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in -20..=20 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn min_cht_monotonic_queries() {
        // Slopes: 3 > 2 > 1 > 0
        let functions = vec![(3, -100), (2, 0), (1, 50), (0, 100)];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        // x is monotonically increasing.
        for x in -100..=100 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_monotonic_queries() {
        // Slopes: 0 < 1 < 2 < 3
        let functions = vec![(0, -100), (1, 50), (2, 0), (3, -100)];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        // x is monotonically increasing.
        for x in -100..=100 {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn min_cht_queries_at_intersections() {
        // Slopes: 3 > 2 > 1
        //
        // f1 = 3x
        // f2 = 2x + 10
        // f3 = x + 20
        //
        // Intersections:
        // f1 = f2 -> x = 10
        // f2 = f3 -> x = 10
        let functions = vec![(3, 0), (2, 10), (1, 20)];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in [-10, 0, 9, 10, 11, 20] {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_queries_at_intersections() {
        // Slopes: 1 < 2 < 3
        let functions = vec![(1, 20), (2, 10), (3, 0)];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in [-10, 0, 9, 10, 11, 20] {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn min_cht_large_values() {
        // Slopes: 2_000_000 > 1_000_000 > -1_000_000
        let functions = vec![
            (2_000_000, -1_000_000),
            (1_000_000, 1_000_000),
            (-1_000_000, 2_000_000),
        ];

        let mut cht = ConvexHullTrick::<false>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in [-1_000_000, -1000, 0, 1000, 1_000_000] {
            assert_eq!(
                cht.evaluate(x),
                brute_force_min(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }

    #[test]
    fn max_cht_large_values() {
        // Slopes: -1_000_000 < 1_000_000 < 2_000_000
        let functions = vec![
            (-1_000_000, 2_000_000),
            (1_000_000, 1_000_000),
            (2_000_000, -1_000_000),
        ];

        let mut cht = ConvexHullTrick::<true>::new();

        for &f in &functions {
            cht.add(f);
        }

        for x in [-1_000_000, -1000, 0, 1000, 1_000_000] {
            assert_eq!(
                cht.evaluate(x),
                brute_force_max(&functions, x),
                "wrong answer for x = {x}"
            );
        }
    }
}
