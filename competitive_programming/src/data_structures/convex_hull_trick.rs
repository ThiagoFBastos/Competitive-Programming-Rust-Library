use std::collections::VecDeque;

/// Data structure implementing the Convex Hull Trick, used to answer
/// minimum or maximum queries over a set of linear functions
/// `f(x) = a*x + b` efficiently.
///
/// The const generic parameter `IS_MAX_CHT` selects the mode of operation:
/// `false` maintains the lower hull (minimum queries) and `true` maintains
/// the upper hull (maximum queries).
///
/// # Invariant
/// Functions must be inserted via [`ConvexHullTrick::add`] in monotonic
/// order of slope; otherwise the hull maintained by the structure becomes
/// incorrect.
pub struct ConvexHullTrick<const IS_MAX_CHT: bool> {
    cht: VecDeque<(i64, i64)>,
}

impl<const IS_MAX_CHT: bool> Default for ConvexHullTrick<IS_MAX_CHT> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Evaluation {
    /// Evaluates the hull at `x`, returning the minimum or maximum value
    /// (depending on `IS_MAX_CHT`) among all inserted functions.
    fn evaluate(&mut self, x: i64) -> i64;
}

/// Checks whether the middle function `b` is rendered redundant (overshadowed)
/// by inserting a new function `c` after `a` and `b`.
///
/// Returns `true` if `b` can be safely removed from the convex hull deque.
///
/// # Arguments
/// * `a` - The second-to-last line currently in the convex hull (index `len - 2`).
/// * `b` - The last line currently in the convex hull (index `len - 1`).
/// * `c` - The new line to be added.
///
/// # Preconditions
/// Assumes that lines are being added in monotonic order of their slopes.
fn overshadow(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
    // Equal-slope cases are handled separately because the cross-product
    // comparison below assumes non-zero denominators.
    if a.0 == b.0 || b.0 == c.0 {
        return true;
    }

    // Compares the intersections of (a,b) and (b,c) via cross multiplication
    // (i128 avoids overflow) instead of division, to preserve integer precision.
    let num1 = (b.1 - a.1) as i128;
    let den1 = (a.0 - b.0) as i128;

    let num2 = (c.1 - b.1) as i128;
    let den2 = (b.0 - c.0) as i128;

    num1 * den2 >= num2 * den1
}

fn value_of(function: (i64, i64), x: i64) -> i64 {
    function.0 * x + function.1
}

impl<const IS_MAX_CHT: bool> ConvexHullTrick<IS_MAX_CHT> {
    pub fn new() -> Self {
        Self {
            cht: VecDeque::new(),
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        Self {
            cht: VecDeque::with_capacity(n),
        }
    }

    pub fn clear(&mut self) {
        self.cht.clear();
    }

    /// Inserts `function` into the hull, popping from the back any
    /// functions that become redundant as a result (see [`overshadow`]).
    ///
    /// # Precondition
    /// `function` must have a monotonic slope relative to the previously
    /// inserted functions (see the invariant on [`ConvexHullTrick`]).
    pub fn add(&mut self, function: (i64, i64)) {
        let mut len = self.cht.len();

        while len >= 2 && overshadow(self.cht[len - 2], self.cht[len - 1], function) {
            len -= 1;
            self.cht.pop_back();
        }

        self.cht.push_back(function);
    }
}

impl Evaluation for ConvexHullTrick<false> {
    /// Returns `i64::MAX` if the hull is empty.
    ///
    /// Pops from the front of the deque any functions that are no longer
    /// optimal at `x` (the answer is monotonic across increasing query
    /// values of `x`, which allows permanent eviction instead of a binary
    /// search).
    fn evaluate(&mut self, x: i64) -> i64 {
        if self.cht.is_empty() {
            return i64::MAX;
        }

        while self.cht.len() >= 2 {
            let v1 = value_of(self.cht[0], x);
            let v2 = value_of(self.cht[1], x);

            if v1 < v2 {
                break;
            }

            self.cht.pop_front();
        }

        let f = self.cht[0];

        value_of(f, x)
    }
}

impl Evaluation for ConvexHullTrick<true> {
    /// Returns `i64::MIN` if the hull is empty.
    ///
    /// Analogous to [`ConvexHullTrick<false>::evaluate`], but evicts from
    /// the front any functions that are no longer maximal at `x`.
    fn evaluate(&mut self, x: i64) -> i64 {
        if self.cht.is_empty() {
            return i64::MIN;
        }

        while self.cht.len() >= 2 {
            let v1 = value_of(self.cht[0], x);
            let v2 = value_of(self.cht[1], x);

            if v1 > v2 {
                break;
            }

            self.cht.pop_front();
        }

        let f = self.cht[0];

        value_of(f, x)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

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

        let mut cht = ConvexHullTrick::<false>::with_capacity(functions.len());

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

        cht.clear();

        assert_eq!(cht.evaluate(0), i64::MAX);
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
