/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Simple Sparse Table data structure that handles range minimum queries.
 */

struct SparseTable<T> {
   st: Vec<Vec<T>>
}

impl<T: Ord + Copy> SparseTable<T> {

    /**
     * create a new instance of SparseTable
     * @param values an array with the elements used by the data structure
     * a new SparseTable 
     */
   fn new(values: &[T]) -> Self {
      let n = values.len();
      
      assert!(n > 0, "The array must have positive length");

      let k = 32 - (n as i32).leading_zeros() as usize;

      let mut st = (0..k).map(|_| Vec::with_capacity(n)).collect::<Vec<_>>();

      for i in 0..n {
         st[0].push(values[i]);
      }

      for i in 1..k {
         for j in 0..n {
            if j + (1 << i) > n {
               break;
            }

            let min_value = st[i - 1][j].min(st[i - 1][j + (1 << (i - 1))]);

            st[i].push(min_value);
         }
      }

      Self {
         st: st
      }
   }

   /**
    * return the min value between the l and r positions.
    * @param l the leftmost position of interval
    * @param r the rightmost position of interval
    * @return the min value of type T
    */
   fn query(&self, l: usize, r: usize) -> T {
      assert!(l <= r, "The interval must be non-degenerate");

      let log = 31 - ((r - l + 1) as i32).leading_zeros() as usize;

      self.st[log][l].min(self.st[log][r + 1 - (1 << log)])
   }
}
