/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Union Find data structure that solves the problem
 * of to connect two vertexes that a(u) := a(v) + x (mod 998244353) and given u, v 
 * we have to find a(u) - a(v) (mod 998244353).
 */

const MOD: i32 = 998244353;

#[derive(Clone)]
struct DisjointSet {
   parent: Vec<usize>,
   rank: Vec<u32>,
   up: Vec<i32>
}

impl DisjointSet {

    /**
     * create a new instance of DisjointSet
     * @param n the number of vertexes of the disjoint set
     * @return the new DisjointSet data structure
     */
   fn new(n: usize) -> Self {

      Self {
         parent: (0..n).collect(),
         rank: vec![0; n],
         up: vec![0; n]
      }
   }

   /**
    * find the root of the disjoint set and the sum of weights from the source vertex to it
    * @param u the source vertex
    * @return a tuple containing the root and the sum of weights
    */
   fn find_set(&self, mut u: usize) -> (usize, i32) {
      let mut weight = 0;

      while u != self.parent[u] {
         weight += self.up[u];
         if weight >= MOD {
            weight -= MOD;
         }
         u = self.parent[u];
      }

      (u, weight)
   }

    /**
     * check if the given two vertexes belongs to the same connected component
     * @param u a vertex of a connected component
     * @param v a vertex of a connected component
     * @return true if u and v belongs to the same connected component and false otherwise
    */ 
   fn same(&self, u: usize, v: usize) -> bool {
      let (a, _) = self.find_set(u);
      let (b, _) = self.find_set(v);

      return a == b;
   }

    /**
     * join the roots of the disjoint set of the given vertexes
     * @param u the vertex of a disjoint set
     * @param v the vertex of a disjoint set
     * @param weight the weight of the edge (u, v)
     * @return true if the vertexes are joined, just now, and false otherwise
     */ 
   fn unite(&mut self, u: usize, v: usize, mut weight: i32) -> bool {
      let (mut a, mut wa) = self.find_set(u);
      let (mut b, mut wb) = self.find_set(v);

      wa *= -1;

      if a == b {
         return false;
      } else if self.rank[a] > self.rank[b] {
         weight *= -1;
         wa *= -1;
         wb *= -1;
         std::mem::swap(&mut a, &mut b);
         std::mem::swap(&mut wa, &mut wb);
      }

      self.parent[a] = b;
      self.up[a] = (((wa + wb) % MOD + weight) % MOD + MOD) % MOD;
      self.rank[b] = if self.rank[a] == self.rank[b] { 1 + self.rank[b] } else { self.rank[b] };

      return true;
   }

   /**
    * return the height of a given vertex to the root of the disjoint set
    * @param u the target vertex
    * @return the height of the vertex
    */
   fn height(&self, mut u: usize) -> i32 {
      let mut h = 0;

      while u != self.parent[u] {
         u = self.parent[u];
         h += 1;
      }

      return h;
   }

   /**
    * return a(u) - a(v) (mod 998244353)
    * @param u the source vertex
    * @param v the destination vertex
    */
   fn distance(&self, mut u: usize, mut v: usize) -> i32 {
      let mut hu = self.height(u);
      let mut hv = self.height(v);
      let mut distance = 0;

      while u != v {
         if hu > hv {
            distance += self.up[u];

            if distance >= MOD {
               distance -= MOD;
            }

            hu -= 1;
            u = self.parent[u];
         } else if hu < hv {
            distance -= self.up[v];

            if distance < 0 {
               distance += MOD;
            }

            hv -= 1;
            v = self.parent[v];
         } else {
            distance += self.up[u] - self.up[v];

            if distance >= MOD {
               distance -= MOD;
            }

            if distance < 0 {
               distance += MOD;
            }

            u = self.parent[u];
            v = self.parent[v];
         }
      }

      return distance;
   }
}
