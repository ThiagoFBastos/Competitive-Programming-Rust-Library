use crate::graphs::tree_center;
use std::collections::BTreeMap;

type Graph = Vec<Vec<usize>>;

pub struct Ahu {
    pattern: BTreeMap<Vec<i32>, i32>, // patterns of the vertices
}

impl Default for Ahu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ahu {
    pub fn new() -> Self {
        Self {
            pattern: BTreeMap::new(),
        }
    }

    fn dfs(&mut self, g: &Graph, u: usize, p: Option<usize>) -> i32 {
        let mut patterns = Vec::new();

        for &v in g[u].iter() {
            if Some(v) != p {
                patterns.push(self.dfs(g, v, Some(u)));
            }
        }

        patterns.sort();

        if !self.pattern.contains_key(&patterns) {
            let id = self.pattern.len() as i32;

            self.pattern.insert(patterns, id);

            return id;
        }

        *self.pattern.get(&patterns).unwrap()
    }

    /**
     * Find a unique pattern of the rooted tree
     */
    pub fn get_tree_pattern(&mut self, g: &Graph, root: usize) -> i32 {
        assert_ne!(g.len(), 0);
        self.dfs(g, root, None)
    }

    /**
     * @brief Find a unique pattern of the tree
     */
    pub fn get_tree_canonical_pattern(&mut self, g: &Graph) -> i32 {
        let mut max_pattern = 0;

        if !g.is_empty() {
            for v in tree_center(g) {
                max_pattern = max_pattern.max(self.get_tree_pattern(g, v));
            }
        }

        max_pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    type Graph = Vec<Vec<usize>>;

    fn add_edge(g: &mut Graph, u: usize, v: usize) {
        g[u].push(v);
        g[v].push(u);
    }

    fn build_tree(parents: &[usize]) -> Graph {
        let n = parents.len() + 1;
        let mut g = vec![vec![]; n];

        for (i, &p) in parents.iter().enumerate() {
            let u = i + 1;
            let v = p - 1;

            g[u].push(v);
            g[v].push(u);
        }

        g
    }

    #[test]
    fn empty_trees_test() {
        let adj1 = Vec::new();
        let adj2 = Vec::new();

        let mut ahu = Ahu::new();

        let pattern_adj1 = ahu.get_tree_canonical_pattern(&adj1);
        let pattern_adj2 = ahu.get_tree_canonical_pattern(&adj2);

        assert_eq!(pattern_adj1, pattern_adj2);
    }

    #[test]
    fn star_tree_test() {
        const N: usize = 8;

        let mut adj1 = vec![vec![]; N];
        let mut adj2 = vec![vec![]; N];

        for i in 0..N {
            if i != 0 {
                add_edge(&mut adj1, 0, i);
            }

            if i != 1 {
                add_edge(&mut adj2, 1, i);
            }
        }

        let mut ahu = Ahu::new();

        let pattern_adj1 = ahu.get_tree_canonical_pattern(&adj1);
        let pattern_adj2 = ahu.get_tree_canonical_pattern(&adj2);

        assert_eq!(pattern_adj1, pattern_adj2);
    }

    #[test]
    fn different_trees_test() {
        const N: usize = 8;

        let mut adj1 = vec![vec![]; N];
        let mut adj2 = vec![vec![]; N];

        let mut ahu = Ahu::new();

        for i in 1..N {
            add_edge(&mut adj1, 0, i);
            add_edge(&mut adj2, i - 1, i);
        }

        let pattern_adj1 = ahu.get_tree_canonical_pattern(&adj1);
        let pattern_adj2 = ahu.get_tree_canonical_pattern(&adj2);

        assert_ne!(pattern_adj1, pattern_adj2);
    }

    #[test]
    fn count_unique_tree_patterns_test() {
        let data: Vec<Vec<usize>> = vec![
            vec![1, 1, 1, 1, 3],
            vec![1, 2, 1, 2, 5],
            vec![1, 2, 1, 2, 4],
            vec![1, 2, 3, 1, 1],
            vec![1, 2, 2, 1, 4],
            vec![1, 2, 1, 3, 5],
            vec![1, 2, 1, 3, 5],
            vec![1, 1, 2, 4, 2],
            vec![1, 1, 1, 2, 4],
            vec![1, 2, 1, 4, 1],
            vec![1, 2, 2, 3, 4],
            vec![1, 2, 1, 3, 4],
            vec![1, 1, 1, 3, 4],
            vec![1, 2, 3, 1, 2],
            vec![1, 2, 3, 1, 2],
            vec![1, 1, 3, 2, 5],
            vec![1, 1, 2, 1, 3],
            vec![1, 2, 1, 3, 1],
            vec![1, 1, 3, 3, 5],
            vec![1, 1, 3, 3, 3],
            vec![1, 2, 1, 2, 3],
            vec![1, 2, 2, 4, 3],
            vec![1, 1, 2, 1, 3],
            vec![1, 2, 2, 3, 1],
            vec![1, 1, 3, 3, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 2, 3, 4, 4],
            vec![1, 1, 2, 3, 1],
            vec![1, 2, 3, 3, 3],
            vec![1, 2, 1, 2, 3],
            vec![1, 1, 3, 2, 3],
            vec![1, 2, 2, 4, 1],
            vec![1, 1, 1, 1, 5],
            vec![1, 2, 3, 3, 4],
            vec![1, 2, 3, 2, 2],
            vec![1, 1, 2, 2, 4],
            vec![1, 2, 3, 4, 2],
            vec![1, 2, 2, 2, 1],
            vec![1, 1, 2, 2, 5],
            vec![1, 1, 2, 3, 5],
        ];

        let expected_trees = 5;

        let mut ahu = Ahu::new();
        let mut set = HashSet::new();

        for parents in data {
            let g = build_tree(&parents);
            let pattern = ahu.get_tree_canonical_pattern(&g);
            set.insert(pattern);
        }

        assert_eq!(set.len(), expected_trees);
    }
}
