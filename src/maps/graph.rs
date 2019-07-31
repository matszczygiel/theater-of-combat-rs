use std::collections::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Default, Debug, Clone)]
pub struct BidirectionalGraph<T: Debug + Clone + Hash + Eq + Default> {
    graph: HashMap<T, HashSet<T>>,
}

impl<T: Debug + Clone + Hash + Eq + Default> BidirectionalGraph<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_node(&mut self, node: T, neighbors: HashSet<T>) -> Result<(), &'static str> {
        for n in neighbors.iter() {
            self.graph
                .get_mut(n)
                .ok_or("There is no node corresponding to one of the neighbors.")?
                .insert(node.clone());
        }

        if self.graph.insert(node, neighbors).is_some() {
            return Err("Reinserting node into the graph.");
        }

        Ok(())
    }

    pub fn remove_node(&mut self, node: T) -> Result<(), &'static str> {
        self.graph
            .remove(&node)
            .ok_or("Removing nonexistent node.")?;

        for (_, v) in self.graph.iter_mut() {
            v.remove(&node);
        }

        Ok(())
    }
}

impl<T: Debug + Clone + Hash + Eq + Default> Eq for BidirectionalGraph<T> {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserting_test() {
        let mut bg = BidirectionalGraph::default();
        bg.insert_node(0, HashSet::default()).unwrap();
        bg.insert_node(1, HashSet::default()).unwrap();
        bg.insert_node(2, [1].iter().cloned().collect()).unwrap();
        bg.insert_node(3, [1, 2].iter().cloned().collect()).unwrap();
        bg.insert_node(4, [0, 3].iter().cloned().collect()).unwrap();

        let expected: HashMap<i32, HashSet<i32>> = [
            (0, [4].iter().cloned().collect()),
            (1, [2, 3].iter().cloned().collect()),
            (2, [1, 3].iter().cloned().collect()),
            (3, [1, 2, 4].iter().cloned().collect()),
            (4, [0, 3].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(bg.graph, expected);
    }

    #[test]
    fn removing_test() {
        let mut bg = BidirectionalGraph::default();
        bg.insert_node(0, HashSet::default()).unwrap();
        bg.insert_node(1, HashSet::default()).unwrap();
        bg.insert_node(2, [1].iter().cloned().collect()).unwrap();
        bg.insert_node(3, [1, 2].iter().cloned().collect()).unwrap();
        bg.insert_node(4, [0, 3].iter().cloned().collect()).unwrap();

        bg.remove_node(4).unwrap();

        let expected: HashMap<i32, HashSet<i32>> = [
            (0, HashSet::default()),
            (1, [2, 3].iter().cloned().collect()),
            (2, [1, 3].iter().cloned().collect()),
            (3, [1, 2].iter().cloned().collect()),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(bg.graph, expected);
    }

}
