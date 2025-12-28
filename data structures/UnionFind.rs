/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Simple implementation of Union Find data structure.
 */

/*
  Simple Disjoint Set/Union Find with rank and path compression
*/
struct DisjointSet {
   parent: Vec<usize>,
   rank: Vec<u32>
}

impl DisjointSet {
    /**
     * create a new instance of DisjointSet
     * @param n number of vertexes
     * @return a DisjointSet
     */
   fn new(n: usize) -> Self {

      Self {
         parent: (0..n).collect(),
         rank: vec![0; n]
      }
   }

   /**
    * find the root of disjoint set that u belongs
    * @param u a vertex of the disjoint set that you want to find the root
    * @return the root of disjoint set
    */
   fn find_set(&mut self, u: usize) -> usize {
      
      if u == self.parent[u] {
         return u;
      }

      self.parent[u] = self.find_set(self.parent[u]);

      return self.parent[u];
   }

   /**
    * join the connected components that u and v belongs
    * @param u a vertex of one connected component
    * @param v a vertex of other connected component
    */
   fn unite(&mut self, mut u: usize, mut v: usize) {

      u = self.find_set(u);
      v = self.find_set(v);

      if u == v {
         return;
      } else if self.rank[u] > self.rank[v] {
         std::mem::swap(&mut u, &mut v);
      }

      self.parent[u] = v;
      self.rank[v] = if self.rank[u] == self.rank[v] { self.rank[v] + 1 } else { self.rank[v] };
   }

   /**
    * Return if two vertexes belongs to the same connected component
    * @param u a vertex of one connected component
    * @param v a vertex of one connected component
    * @return true if u and v belongs to the same connected component and false otherwise
    */
   fn same(&mut self, u: usize, v: usize) -> bool {
      return self.find_set(u) == self.find_set(v);
   }
}
