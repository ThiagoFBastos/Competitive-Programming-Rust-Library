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
