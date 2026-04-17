#[cfg(test)]
mod tests {
    use competitive_programming::data_structures::SparseTable;

    #[test]
    fn test_range_min_query() {
        let min = |a: usize, b: usize| a.min(b);

        const N: usize = 10;

        let values = (1..=N).collect::<Vec<_>>();

        let sp = SparseTable::new(&values, min);

        for i in 0..N {
            for j in i..N {
                let value = sp.query(i, j);
                assert_eq!(value, i + 1);
            }
        }
    }

    #[test]
    fn test_range_max_query() {
        let max = |a: usize, b: usize| a.max(b);

        const N: usize = 10;

        let values = (1..=N).collect::<Vec<_>>();

        let sp = SparseTable::new(&values, max);

        for i in 0..N {
            for j in i..N {
                let value = sp.query(i, j);
                assert_eq!(value, j + 1);
            }
        }
    }

    #[test]
    fn test_range_or_query() {
        let or = |a: usize, b: usize| a | b;

        const N: usize = 10;

        let values = (1..=N).collect::<Vec<_>>();

        let sp = SparseTable::new(&values, or);

        for i in 0..N {
            let mut or_result = 0;

            for (j, item) in values.iter().enumerate().take(N).skip(i) {
                let value = sp.query(i, j);

                or_result |= item;

                assert_eq!(value, or_result);
            }
        }
    }
}
