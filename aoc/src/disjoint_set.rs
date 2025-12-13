use std::collections::HashMap;

/// DisjointSet is collection of values where each value belongs to some unnamed
/// set. You can add new values as a singleton set, and merge two sets together
/// by providing values that are in those sets.
///
/// Each set is represented as a tree, with each node pointing to its parent,
/// and the root pointing to itself. The size is tracked for each set and can be
/// queried.
#[derive(Debug, Clone)]
pub struct DisjointSet {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    lens: Vec<usize>,
}

impl DisjointSet {
    /// Empty set.
    pub fn new() -> Self {
        Self {
            parents: vec![],
            ranks: vec![],
            lens: vec![],
        }
    }

    /// Create with `n` single element sets.
    pub fn with_singles(n: usize) -> Self {
        let mut i = 0;
        Self {
            parents: std::iter::from_fn(|| {
                i += 1;
                Some(i - 1)
            })
            .take(n)
            .collect(),
            ranks: vec![0; n],
            lens: vec![1; n],
        }
    }

    /// Insert a new singleton set. Returns the identity/root of that set.
    pub fn insert_single(&mut self) -> usize {
        self.parents.push(self.parents.len());
        self.ranks.push(0);
        self.lens.push(1);
        self.parents.len() - 1
    }

    /// Given two elements maybe belonging to different sets, merge the two sets
    /// they belong to into a single set.
    pub fn merge(&mut self, left: usize, right: usize) {
        let lr = self.find_root(left);
        let rr = self.find_root(right);

        if lr == rr {
            return;
        }

        if self.ranks[lr] < self.ranks[rr] {
            self.parents[lr] = rr;
            self.ranks[rr] += 1;
            self.lens[rr] += self.lens[lr];
        } else {
            self.parents[rr] = lr;
            if self.ranks[lr] == self.ranks[rr] {
                self.ranks[lr] += 1;
            }
            self.lens[lr] += self.lens[rr];
        }
    }

    /// Get the length of the set that the query element is part of.
    pub fn len_of(&self, query: usize) -> usize {
        let root = self.find_root_const(query);
        self.lens[root]
    }

    /// Get a list of all set sizes. Not sorted.
    pub fn all_lens(&self) -> Vec<usize> {
        let mut sizes = HashMap::<usize, usize>::new();
        for i in 0..self.parents.len() {
            let root = self.find_root_const(i);
            sizes.insert(root, self.lens[root]);
        }

        sizes.into_values().collect()
    }

    /// Find the root element of a set from a member of that set. Flatten the
    /// tree as we go.
    fn find_root(&mut self, query: usize) -> usize {
        if self.parents[query] != query {
            self.parents[query] = self.find_root(self.parents[query]);
            self.parents[query]
        } else {
            query
        }
    }

    // Same as `find_root` except we don't update the tree.
    fn find_root_const(&self, query: usize) -> usize {
        if self.parents[query] != query {
            self.find_root_const(self.parents[query])
        } else {
            query
        }
    }
}

#[cfg(test)]
mod test {
    use crate::DisjointSet;

    #[test]
    fn init() {
        let mut s = DisjointSet::new();
        let i = s.insert_single();
        assert_eq!(0, i);
        assert_eq!(s.find_root(0), 0);
        assert_eq!(s.len_of(0), 1);
    }

    #[test]
    fn merge() {
        let mut s = DisjointSet::new();
        let i1 = s.insert_single();
        let i2 = s.insert_single();
        let i3 = s.insert_single();
        s.merge(i1, i2);

        assert_eq!(s.find_root(i1), s.find_root(i2));
        assert_ne!(s.find_root(i1), s.find_root(i3));
        assert_eq!(s.len_of(i1), 2);
        assert_eq!(s.len_of(i2), 2);
        assert_eq!(s.len_of(i3), 1);
    }

    #[test]
    fn larger() {
        let mut s = DisjointSet::with_singles(10);
        s.merge(4, 5);
        s.merge(4, 6);

        assert_eq!(s.find_root(4), s.find_root(6));
        assert_eq!(s.len_of(4), 3);
    }

    #[test]
    fn balanced_merge() {
        let mut s = DisjointSet::with_singles(100);
        for win in (0..100).into_iter().collect::<Vec<_>>().windows(2) {
            s.merge(win[0], win[1]);
        }

        assert!(s.parents.iter().all(|p| *p == 0));
        assert_eq!(s.len_of(50), 100);
    }
}
