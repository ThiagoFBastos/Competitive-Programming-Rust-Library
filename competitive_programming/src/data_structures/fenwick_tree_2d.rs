use std::ops::{Add, AddAssign, Sub};

pub struct FenwickTree2D<T> {
    data: Vec<Vec<T>>,
    pub rows: usize,
    pub columns: usize,
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
    fn query(&self, mut x: i32, mut y: i32) -> T {
        x += 1;
        y += 1;

        let mut answer = T::default();

        while x > 0 {
            let mut k = y;

            while k > 0 {
                answer += self.data[x as usize][k as usize];
                k -= k & -k;
            }

            x -= x & -x;
        }

        answer
    }

    /**
     * calculate the sum of all values inside the submatrix (x0, y0) x (x1, y1)
     */
    pub fn calculate(&self, x0: i32, y0: i32, x1: i32, y1: i32) -> T {
        self.query(x1, y1) - self.query(x1, y0 - 1) - self.query(x0 - 1, y1)
            + self.query(x0 - 1, y0 - 1)
    }

    /**
     * add a value to (x, y) position of the matrix
     * @param x the row position of the matrix
     * @param y the column position of the matrix
     * @param value the number that will be added to position (x, y)
     */
    pub fn update(&mut self, mut x: i32, mut y: i32, value: T) {
        x += 1;
        y += 1;

        while x as usize <= self.rows {
            let mut k = y;

            while k as usize <= self.columns {
                self.data[x as usize][k as usize] += value;
                k += k & -k;
            }

            x += x & -x;
        }
    }
}
