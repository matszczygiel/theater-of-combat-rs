use std::collections::*;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct BidirectionalGraph<T: Debug + Clone + Hash + Eq + PartialEq + Default> {
    graph: HashMap<T, HashSet<T>>,
}

impl<T: Debug + Clone + Hash + Eq + Default> BidirectionalGraph<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_node(
        &mut self,
        node: T,
        neighbors: HashSet<T>,
    ) -> Result<&mut Self, &'static str> {
        for n in neighbors.iter() {
            self.graph
                .get_mut(n)
                .ok_or("There is no node corresponding to one of the neighbors.")?
                .insert(node.clone());
        }

        if self.graph.insert(node, neighbors).is_some() {
            return Err("Reinserting node into the graph.");
        }

        Ok(self)
    }

    pub fn remove_node(&mut self, node: T) -> Result<&mut Self, &'static str> {
        self.graph
            .remove(&node)
            .ok_or("Removing nonexistent node.")?;

        for v in self.graph.values_mut() {
            v.remove(&node);
        }

        Ok(self)
    }

    pub fn get_map(&self) -> &HashMap<T, HashSet<T>> {
        &self.graph
    }
}
/// Dijkstra shortest path algorithm for bidirectional map represented as HashMap.
/// Weights keys contain the nodes indices, values the weight.
/// Returns map of distances and map of the previous node.
pub fn dijkstra(
    graph: &HashMap<i32, HashSet<i32>>,
    source: i32,
    weights: &HashMap<i32, i32>,
) -> Result<(HashMap<i32, i32>, HashMap<i32, i32>), &'static str> {
    const INFINITY: i32 = std::i32::MAX;

    let mut dist = HashMap::new();
    for node in graph.keys() {
        dist.insert(*node, INFINITY);
    }
    dist.insert(source, 0);

    let mut queue = BinaryHeap::new();
    queue.push((dist[&source], source));

    let mut prev = HashMap::new();

    while !queue.is_empty() {
        let u = queue.pop().unwrap().1;
        for v in graph[&u].iter().copied() {
            let alt = dist[&u]
                + *weights
                    .get(&v)
                    .ok_or("Nodes in the graph doesn't match keys in weights.")?;
            if alt < dist[&v] {
                dist.insert(v, alt);
                prev.insert(v, u);
                queue.push((dist[&v], v));
            }
        }
    }

    Ok((dist, prev))
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

    #[test]
    fn test_dijkstra() {
        let mut bg = BidirectionalGraph::default();
        bg.insert_node(0, HashSet::default())
            .unwrap()
            .insert_node(1, [0].iter().cloned().collect())
            .unwrap()
            .insert_node(2, [1].iter().cloned().collect())
            .unwrap()
            .insert_node(3, [1, 2].iter().cloned().collect())
            .unwrap()
            .insert_node(4, [0, 3].iter().cloned().collect())
            .unwrap();

        let mut wg = HashMap::new();
        wg.insert(0, 3);
        wg.insert(1, 4);
        wg.insert(2, 3);
        wg.insert(3, 0);
        wg.insert(4, 1);

        let (dist, prev) = dijkstra(bg.get_map(), 3, &wg).unwrap();

        println!("dist:\n {:?}", dist);
        println!("prev:\n {:?}", prev);
        println!("graph: \n {:?}", bg);

        assert_eq!(dist[&0], 4);
        assert_eq!(dist[&1], 4);
        assert_eq!(dist[&2], 3);
        assert_eq!(dist[&3], 0);
        assert_eq!(dist[&4], 1);

        assert_eq!(prev[&0], 4);
        assert_eq!(prev[&1], 3);
        assert_eq!(prev[&2], 3);
        assert_eq!(prev[&4], 3);
    }

}
