/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Simple Sparse Table data structure that handles static range queries.
 */

mod data_structures {

   pub mod ranges {

      pub struct SparseTable<T, OP> where OP: Fn(T, T) -> T {
         st: Vec<Vec<T>>, // the sparse table data 
         op: OP // the binary function to get the values of queries  
      }

      impl<T: Ord + Copy, OP> SparseTable<T, OP> where OP: Fn(T, T) -> T {

         /**
          * Create a new instance of SparseTable
          * @param values an array with the elements used by the data structure
          * a new SparseTable 
          */
         pub fn new(values: &[T], op: OP) -> Self {
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

                  let value = op(st[i - 1][j], st[i - 1][j + (1 << (i - 1))]);

                  st[i].push(value);
               }
            }

            Self {
               st,
               op
            }
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
   }
}
