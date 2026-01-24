/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-30
 * Description: Simple recursive segment tree agnostic to operations.
 */

pub struct SegTree<T, OP> where OP: Fn(T, T) -> T {
   data: Vec<T>, // the segment tree data
   size: usize, // the number of elements
   op: OP // the binary operator
}

impl<T: Copy + Default, OP> SegTree<T, OP> where OP: Fn(T, T) -> T {

   /**
   * Create a new instance of SegTree
   * @param values the initial array of elements
   * @param op the binary operator
   */
   pub fn new(values: &[T], op: OP) -> Self {
      let n = values.len();

      let mut st = Self {
         data: vec![T::default(); 4 * n],
         size: n,
         op
      };

      st.build(values, 0, n - 1, 1);

      return st;
   }

   /**
    * Build the segment tree
      * @param values the initial array of elements
      * @param lo the leftmost position of the current range node
      * @param hi the rightmost position of the current range node
      * @param p the current node 
      */
   fn build(&mut self, values: &[T], lo: usize, hi: usize, p: usize) {
      if lo == hi {
         self.data[p] = values[lo];
         return;
      }

      let mid = (lo + hi) / 2;

      self.build(values, lo, mid, 2 * p);
      self.build(values, mid + 1, hi, 2 * p + 1);

      self.data[p] = (self.op)(self.data[2 * p], self.data[2 * p + 1]);
   }

   /**
    * Update the value at a specific position
      * @param k the position to update
      * @param value the value that will replace the old one
      * @param lo the leftmost position of the current range node
      * @param hi the rightmost position of the current range node
      * @param p the current node 
      */
   fn change(&mut self, k: usize, value: T, lo: usize, hi: usize, p: usize) {
      if lo == hi {
         self.data[p] = value;
         return;
      }

      let mid = (lo + hi) / 2;

      if k <= mid {
         self.change(k, value, lo, mid, 2 * p);
      } else {
         self.change(k, value, mid + 1, hi, 2 * p + 1);
      }

      self.data[p] = (self.op)(self.data[2 * p], self.data[2 * p + 1]);
   }

   /**
    * Find the value of an operation over a range
      * @param l the leftmost position of the target range
      * @param r the rightmost position of the target range
      * @param lo the leftmost position of the current range node
      * @param hi the rightmost position of the current range node
      * @param p the current node 
      */
   fn answer(&self, l: usize, r: usize, lo: usize, hi: usize, p: usize) -> T {
      if lo >= l && hi <= r {
         return self.data[p];
      }

      let mid = (lo + hi) / 2;

      if r <= mid {
         return self.answer(l, r, lo, mid, 2 * p);
      } else if l > mid {
         return self.answer(l, r, mid + 1, hi, 2 * p + 1);
      }

      let res1 = self.answer(l, r, lo, mid, 2 * p);
      let res2 = self.answer(l, r, mid + 1, hi, 2 * p + 1);

      return (self.op)(res1, res2);
   }

   /**
    * Update the value at a specific position
      * @param k the position to update
      * @param value the value that will replace the old one
      */
   pub fn update(&mut self, k: usize, value: T) {
      assert!(k < self.size);
      self.change(k, value, 0, self.size - 1, 1);
   }

   /**
    * Find the value of an operation over a range
      * @param l the leftmost position of the target range
      * @param r the rightmost position of the target range
      */
   pub fn query(&self, l: usize, r: usize) -> T {
      assert!(l <= r && r < self.size);
      return self.answer(l, r, 0, self.size - 1, 1);
   }
}
