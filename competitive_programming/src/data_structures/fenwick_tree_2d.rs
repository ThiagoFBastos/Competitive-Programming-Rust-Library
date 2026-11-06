use std::ops::{Add, AddAssign, Sub};

#[derive(Clone)]
pub struct FenwickTree2D<T> {
    data: Vec<Vec<T>>,  // the fenwick tree data
    pub rows: usize,    // the number of rows
    pub columns: usize, // the number of columns
}

impl<T: Default + Copy + AddAssign + Sub<Output = T> + Add<Output = T>> FenwickTree2D<T> {
    /**
     * create a new instance of FenwickTree2D
     * @param rows the number of rows of the matrix
     * @param columns the number of columns of the matrix
     * @return a new FenwickTree2D
     */
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            data: vec![vec![T::default(); columns + 1]; rows + 1],
            rows,
            columns,
        }
    }

    /**
     * find the sum of all integers inside the submatrix (1, 1) x (x, y)
     * @param x the row position of boundary of the submatrix
     * @param y the column position of boundary of the submatrix
     */
    fn query(&self, mut x: usize, y: usize) -> T {
        assert!(x <= self.rows && y <= self.columns);

        let mut answer = T::default();

        while x > 0 {
            let mut k = y;

            while k > 0 {
                answer += self.data[x][k];
                k -= k & (!k + 1);
            }

            x -= x & (!x + 1);
        }

        answer
    }

    /**
     * calculate the sum of all values inside the submatrix (x0, y0) x (x1, y1)
     */
    pub fn calculate(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> T {
        self.query(x1, y1) - self.query(x1, y0 - 1) - self.query(x0 - 1, y1)
            + self.query(x0 - 1, y0 - 1)
    }

    /**
     * add a value to (x, y) position of the matrix
     * @param x the row position of the matrix
     * @param y the column position of the matrix
     * @param value the number that will be added to position (x, y)
     */
    pub fn update(&mut self, mut x: usize, y: usize, value: T) {
        assert!(x > 0 && y > 0);

        while x <= self.rows {
            let mut k = y;

            while k <= self.columns {
                self.data[x][k] += value;
                k += k & (!k + 1);
            }

            x += x & (!x + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_query() {
        const R: usize = 5;
        const C: usize = 5;

        let mut ft = FenwickTree2D::<i32>::new(R, C);

        for x0 in 1..=R {
            for x1 in x0..=R {
                for y0 in 1..=C {
                    for y1 in y0..=C {
                        let sum = ft.calculate(x0, y0, x1, y1);
                        assert_eq!(sum, 0);
                    }
                }
            }
        }

        for i in 1..=R {
            for j in 1..=C {
                ft.update(i, j, 1); // TODO add random numbers
            }
        }

        for x0 in 1..=R {
            for x1 in x0..=R {
                for y0 in 1..=C {
                    for y1 in y0..=C {
                        let sum = ft.calculate(x0, y0, x1, y1);
                        let rectangle_sum = (x1 - x0 + 1) * (y1 - y0 + 1);
                        assert_eq!(sum, rectangle_sum as i32);
                    }
                }
            }
        }
    }
}
