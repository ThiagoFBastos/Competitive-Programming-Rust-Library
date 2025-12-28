/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Simple Fenwick Tree data structure.
 */

#[derive(Clone)]
struct FenwickTree<T> {
   sum: Vec<T>,
   length: usize
}

impl<T: Default + Copy + AddAssign> FenwickTree<T> {

    /**
     * create a new instance of FenwickTree
     * @param length the number of elements of Fenwick Tree
     * @return the new instance of FenwickTree with zero values 
     */
   fn new(length: usize) -> Self {

      Self {
         sum: vec![T::default(); length + 1],
         length
      }
   }

   /**
    * find the sum of first k elements
    * @param k the number of the first elements for which we want to calculate the sum
    */
   fn query(&self, mut k: i32) -> T {
      let mut sum = T::default();

      assert!(k <= self.length as i32);

      while k > 0 {
         sum += self.sum[k as usize];
         k -= k & -k;
      }

      sum
   }

   /**
    * add a value to the element at position k 
    * @param k the position for which we want to modify
    * @param value the value for which we want to sum
    */
   fn update(&mut self, mut k: i32, value: T) {

      assert!(k > 0);

      while k <= self.length as i32 {
         self.sum[k as usize] += value;
         k += k & -k;
      }
   }
}
