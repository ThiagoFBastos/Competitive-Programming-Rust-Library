#[derive(Clone)]
pub struct SparseTable<T, OP>
where
    OP: Fn(T, T) -> T,
{
    st: Vec<Vec<T>>, // the sparse table data
    op: OP,          // the binary function to get the values of queries
}

impl<T: Ord + Copy, OP> SparseTable<T, OP>
where
    OP: Fn(T, T) -> T,
{
    /**
     * Create a new instance of SparseTable
     * @param values an array with the elements used by the data structure
     * @param op the binary function to calculate the result of an operation
     * a new SparseTable
     */
    pub fn new(values: &[T], op: OP) -> Self {
        let n = values.len();

        assert!(n > 0, "The array must have positive length");

        let k = 32 - (n as i32).leading_zeros() as usize;

        let mut st = (0..k).map(|_| Vec::with_capacity(n)).collect::<Vec<_>>();

        for value in values.iter() {
            st[0].push(*value);
        }

        for i in 1..k {
            for j in 0..n {
                if j + (1 << i) > n {
                    break;
                }

                let value = op(st[i - 1][j], st[i - 1][j + (1 << (i - 1))]);

                st[i].push(value);
            }
        }

        Self { st, op }
    }

    /**
     * Return the result value after apply the binary function op to the interval
     * @param l the leftmost position of interval
     * @param r the rightmost position of interval
     */
    pub fn query(&self, l: usize, r: usize) -> T {
        assert!(l <= r, "The interval must be non-degenerate");

        let log = 31 - ((r - l + 1) as i32).leading_zeros() as usize;

        (self.op)(self.st[log][l], self.st[log][r + 1 - (1 << log)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
