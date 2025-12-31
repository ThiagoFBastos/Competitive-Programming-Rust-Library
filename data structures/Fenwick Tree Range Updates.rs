/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-31
 * Description: Fenwick Tree that can perform range add updates and answer range sums.
 */

mod data_structures {

   pub mod ranges {
    use std::ops::{AddAssign, Mul, Neg, Sub};

      // Trait to convert from usize to the generic type T
      pub trait Conversions {
         fn from_usize(value: usize) -> Self;
      }

      // Trait to provide constants
      pub trait Constants {
         fn zero() -> Self;
         fn one() -> Self;
      }

      #[derive(Clone)]
      pub struct FenwickTreeRange<T> {
         sum: Vec<Vec<T>>, // Fenwick Tree sum data 
         size: usize // number of elements
      }

      impl<T: Copy + Constants + AddAssign + Conversions + Mul<Output = T> + Sub<Output = T> + Neg<Output = T>> FenwickTreeRange<T> {

        /**
         * Create a new instance of FenwickTreeRange
         * @param size the number of elements
         * @return a new FenwickTreeRange with elements having zero values
         */
         pub fn new(size: usize) -> Self {

            Self {
               sum: vec![vec![T::zero(); size + 1]; 2],
               size
            }
         }

         /**
          * Increment the element at position k by value
          * @param bit_pos the index of Fenwick Tree that will be used
          * @param k the position that will be incremeted
          * @param value the value that will be added to position k  
          */
         fn update(&mut self, bit_pos: usize, mut k: i32, value: T) {

            assert!(k >= 1);

            while k <= self.size as i32 {
               self.sum[bit_pos][k as usize] += value;
               k += k & -k;
            }
         }

         /**
          * Return the sum of k first elements of the target Fenwick Tree
          * @param bit_pos the index of Fenwick Tree that will be used
          * @param k the last position from left to right, starting from 1, to sum
          * @return the sum of k first elements
          */
         fn query(&self, bit_pos: usize, mut k: i32) -> T {
            assert!(k <= self.size as i32);

            let mut result = T::zero();

            while k > 0 {
               result += self.sum[bit_pos][k as usize];
               k -= k & -k;
            }

            result
         }

         /**
          * Add a value to each value between l and r
          * @param l the leftmost position of range
          * @param r the rightmost position of range
          * @param value the value to add to each position inside the range
          */
         pub fn add(&mut self, l: usize, r: usize, value: T) {
            self.update(0, l as i32, (T::from_usize(l) - T::one()) * value);
            self.update(0, r as i32 + 1, T::from_usize(r) * -value);
            self.update(1, l as i32, value);
            self.update(1, r as i32 + 1, -value);
         }

         /**
          * Return the sum of k first elements
          * @param k the rightmost position of the prefix that will be added
          * @return the sum of prefix [1, k]
          */
         fn prefix(&self, k: usize) -> T {
            return T::from_usize(k) * self.query(1, k as i32) - self.query(0, k as i32);
         }

         /**
          * Return the sum of elements between l and r
          * @param l the leftmost position of the range
          * @param r the rightmost position of the range
          * @return the sum of the elements inside [l, r]
          */
         pub fn sum(&self, l: usize, r: usize) -> T {
            return self.prefix(r) - self.prefix(l - 1);
         }
      }
   }
}