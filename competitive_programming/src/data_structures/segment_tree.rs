/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-29
 * Description: Simple iterative segment tree agnostic to operations.
 */

// Trait for initial value
pub trait Constants {
   fn initial() -> Self;
}

#[derive(Clone)]
pub struct SegTree<T, OP> where OP: Fn(T, T) -> T {
   data: Vec<T>, // the segment tree data
   length: usize, // the number of elements 
   op: OP // the binary operator
}

impl<T: Constants + Copy, OP> SegTree<T, OP> where OP: Fn(T, T) -> T {

    /**
     * Create a new instance of SegTree
     * @param values the initial array
     * @param op the binary operator to calculate the values
     */
   pub fn new(values: &[T], op: OP) -> Self {

      let n = values.len();
      let mut data = vec![T::initial(); 2 * n];

      for i in n..2*n {
         data[i] = values[i - n];
      }

      for i in (1..n).rev() {
         data[i] = op(data[i << 1], data[(i << 1) | 1]);
      }

      Self {
         data,
         length: n,
         op: op
      }
   }

   /**
    * Find the answer of query between l and r
    * @param l the leftmost position of the interval
    * @param r the rightmost position of the interval
    * @return the query answer of the values inside of the interval [l, r]
    */
   pub fn query(&self, mut l: usize, mut r: usize) -> T {
      let mut answer = T::initial();

      assert!(l <= r && r < self.length);
   
      l += self.length;
      r += self.length;

      while l <= r {

         if l & 1 == 1 {
            answer = (self.op)(answer, self.data[l]);
            l += 1;
         }

         if r & 1 == 0 {
            answer = (self.op)(answer, self.data[r]);
            r -= 1;
         }

         l >>= 1;
         r >>= 1;
      }

      answer
   }

   /**
    * Update a position to replacing the old value by the new one
    * @param k the position to change the value
    * @param value the value that will be replaced at position k
    */
   pub fn update(&mut self, mut k: usize, value: T) {

      assert!(k < self.length);

      k += self.length;

      self.data[k] = value;

      while k > 1 {
         k >>= 1;
         self.data[k] = (self.op)(self.data[k << 1], self.data[(k << 1) | 1]);
      }
   }
}
