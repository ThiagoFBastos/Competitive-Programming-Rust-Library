#[cfg(test)]
mod fenwick_tree_tests {
    use competitive_programming::data_structures::fenwick_tree::*;

    #[test]
    fn test_sum_operations() {
        const N: usize = 10;

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        struct Int32(i32);

        impl Constants for Int32 {
            fn initial() -> Self {
                return Int32(0);
            }
        }

        let sum = |a: Int32, b: Int32| {
            return Int32(a.0 + b.0);
        };

        let mut ft = FenwickTree::new(N, sum);

        for i in 1..=N {
            ft.update(i as i32, Int32(i as i32));
        }

        for i in 1..=N {
            for j in i..=N {
                let sum = (i + j) * (j - i + 1) / 2;
                assert_eq!(Int32(ft.query(j as i32).0 - ft.query(i as i32 - 1).0), Int32(sum as i32));
            }
        }
    }

    #[test]
    fn test_max_operations() {
        const N: usize = 10;

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        struct Int32(i32);

        impl Constants for Int32 {
            fn initial() -> Self {
                return Int32(i32::MIN);
            }
        }

        let max = |a: Int32, b: Int32| {
            return Int32(a.0.max(b.0));
        };

        let mut ft = FenwickTree::new(N, max);

        for i in 1..=N {
            ft.update(i as i32, Int32(i as i32));
        }

        for i in 1..=N {
            assert_eq!(ft.query(i as i32), Int32(i as i32));
        }
    }
}