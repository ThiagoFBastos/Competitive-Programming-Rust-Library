pub trait FenwickTreeConstants {
    fn initial() -> Self; // the initial constant
}

#[derive(Clone)]
pub struct FenwickTree<T, OP> {
    data: Vec<T>,      // the fenwick tree data
    pub length: usize, // the number of elements of the fenwick tree
    op: OP,            // the binary operator to apply an operation in the fenwick tree
}

impl<T: FenwickTreeConstants + Copy, OP: Fn(T, T) -> T> FenwickTree<T, OP> {
    /**
     * create a new instance of FenwickTree
     * @param length the number of elements of the Fenwick Tree
     * @param op the binary function that handles with operations
     * @return the new instance of FenwickTree
     */
    pub fn new(length: usize, op: OP) -> Self {
        Self {
            data: vec![T::initial(); length + 1],
            length,
            op,
        }
    }

    /**
     * find the result of an operation of first k elements
     * @param k the number of the first elements for which we want to find the result
     */
    pub fn query(&self, mut k: usize) -> T {
        let mut result = T::initial();

        assert!(k <= self.length);

        while k > 0 {
            result = (self.op)(result, self.data[k]);
            k -= k & (!k + 1);
        }

        result
    }

    /**
     * update a value to the element at position k
     * @param k the position for which we want to modify
     * @param value the value for which we want to apply
     */
    pub fn update(&mut self, mut k: usize, value: T) {
        assert!(k > 0);

        while k <= self.length {
            self.data[k] = (self.op)(self.data[k], value);
            k += k & (!k + 1);
        }
    }
}
