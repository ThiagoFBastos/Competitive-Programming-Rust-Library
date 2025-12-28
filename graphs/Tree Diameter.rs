/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Given a tree, the algorithm below finds the diameter of the tree.
 */

 /**
  * find the diameter of a given tree
  * @param adj the adjacency list of the tree
  * @return the diameter
  */
fn diameter(adj: &Vec<Vec<usize>>) -> usize {
   let n = adj.len();
 
   let mut deg = vec![0; n];
   let mut dist = vec![0; n];
   let mut queue = VecDeque::new();
 
   for i in 0..n {
      deg[i] = adj[i].len();
 
      if deg[i] == 1 {
         queue.push_back(i);
      }
   }
 
   while !queue.is_empty() {
      let src = queue.pop_front().unwrap();
 
      for &dest in adj[src].iter() {
         deg[dest] -= 1;
 
         if deg[dest] == 1 {
            dist[dest] = 1 + dist[src];
            queue.push_back(dest);
         }
      }
   }
 
   let &d = dist.iter().max().unwrap();
   let count = dist.iter().filter(|&&x| x == d).count();
 
   return 2 * d + count - 1;
}
