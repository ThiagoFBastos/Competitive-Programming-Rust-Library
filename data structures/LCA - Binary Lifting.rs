/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-30
 * Description: Data structure that can answer the lca between two vertexes and the distance of them.
 */

struct LCA {
   adj: Vec<Vec<(usize, i64)>>, // the adjacency list with weights
   depth: Vec<i32>, // the depth/height of each vertex
   st: Vec<Vec<usize>>, // the sparse table for jumps
   weight: Vec<i64> // the sum of weights from the root to each vertex
}

impl LCA {

    /**
     * Create a new instance of LCA data structure
     * @param n the number of nodes
     * @return the new LCA data structure
     */
   fn new(n: usize) -> Self {

      assert_ne!(n, 0);

      let lg = 32 - (n as i32).leading_zeros();

      Self {
         adj: vec![vec![]; n],
         depth: vec![0; n],
         st: vec![vec![0; n]; lg as usize],
         weight: vec![0; n]
      }
   }

   /**
    * Do a DFS from the root to the vertexes to calculate some properties
    * @param u the vertex to do the search
    * @param p the parent of vertex u
    */
   fn dfs(&mut self, u: usize, p: usize) {
      self.st[0][u] = p;

      for i in 1..self.st.len() {
         self.st[i][u] = self.st[i - 1][self.st[i - 1][u]];
      }

      for &(v, w) in self.adj[u].clone().iter() {
         if v == p {
            continue;
         }

         self.depth[v] = 1 + self.depth[u];
         self.weight[v] = w + self.weight[u];
         self.dfs(v, u);         
      }
   }

   /**
    * Build the LCA data structure using a vertex as root
    * @param root the root of the tree
    */
   fn build(&mut self, root: usize) {
      self.depth[root] = 0;
      self.weight[root] = 0;
      self.dfs(root, root);
   }

   /**
    * Add an undirected edge between u and v with weight w, being that w = 1 if the tree is unweighted
    * @param u a vertex of the edge
    * @param v a vertex of the edge
    * @param w the weight of the edge
    */
   fn add_edge(&mut self, u: usize, v: usize, w: i64) {
      self.adj[u].push((v, w));
      self.adj[v].push((u, w));
   }

   /**
    * Find the LCA of two vertexes
    * @param u a vertex of pair to find the lca
    * @param v a vertex of pair to find the lca
    * @return the lca of u and v
    */
   fn query(&self, mut u: usize, mut v: usize) -> usize {
      if self.depth[u] > self.depth[v] {
         std::mem::swap(&mut u, &mut v);
      }

      let height = self.depth[v] - self.depth[u];

      if height != 0 {
         let lg = 31 - height.leading_zeros() as usize;
         
         for i in 0..=lg {
            if (height >> i) & 1 == 1 {
               v = self.st[i][v];
            } 
         }
      }

      if u == v {
         return u;
      }

      let lg = 31 - self.depth[u].leading_zeros() as usize;

      for i in (0..=lg).rev() {
         if self.st[i][u] == self.st[i][v] {
            continue;
         }

         u = self.st[i][u];
         v = self.st[i][v];
      }

      return self.st[0][u];
   }

   /**
    * Calculate the distance between two nodes
    * @param u the vertex of the pair to find the distance
    * @param v the vertex of the pair to find the distance
    * @return the distance between u and v
    */
   fn distance(&self, u: usize, v: usize) -> i64 {
      let x = self.query(u, v);
      return self.weight[u] + self.weight[v] - 2 * self.weight[x];
   }

}
