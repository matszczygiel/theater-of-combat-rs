extern crate sfml;

use crate::graph::*;
use crate::hexagons::*;
use crate::types::*;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
pub struct HexSite {
    coord: HexCoordinates,
    kind: Field,
}

#[derive(Debug, Clone, Copy)]
pub struct RiverSite {
    side1: HexCoordinates,
    side2: HexCoordinates,
    kind: River,
}

#[derive(Debug, Clone, Default)]
pub struct Map {
    graph: BidirectionalGraph<i32>,
    hexes: HashMap<i32, HexSite>,
    rivers: HashMap<i32, RiverSite>,

    current_free_id: i32,
}

impl Map {
    pub fn new() -> Self {
        Map {
            graph: BidirectionalGraph::default(),
            hexes: HashMap::default(),
            rivers: HashMap::default(),
            current_free_id: 0,
        }
    }

    pub fn insert_hex(&mut self, hex: HexSite) -> Result<(), &'static str> {
        if self
            .hexes
            .iter()
            .find(|&(_, h)| {
                if h.coord == hex.coord {
                    return true;
                } else {
                    return false;
                }
            })
            .is_some()
        {
            return Err("Map already contains such hex.");
        }

        let neighbours = hex.coord.neighbors();

        let found_neighbors: HashMap<_, _> = self
            .hexes
            .iter()
            .filter(|&(id, h)| {
                if neighbours.contains(&h.coord) {
                    return true;
                } else {
                    return false;
                }
            })
            .collect();

        self.graph.insert_node(
            self.current_free_id,
            found_neighbors.keys().cloned().cloned().collect(),
        )?;
        self.current_free_id += 1;
        Ok(())
    }

    pub fn insert_river(&mut self, river: RiverSite) -> Result<(), &'static str> {
        let found_hexes: HashMap<_, _> = self
            .hexes
            .iter()
            .filter(|&(_, hex)| {
                if hex.coord == river.side1 || hex.coord == river.side2 {
                    return true;
                } else {
                    return false;
                }
            })
            .collect();

        if found_hexes.len() < 2 {
            return Err("Map doesn't contains such hexes.");
        } else if found_hexes.len() > 2 {
            return Err("Map contains too much such hexes - they must double somehow.");
        }

        let found_rivs = self
            .rivers
            .values()
            .filter(|&riv| {
                if riv.side1 == river.side1 && riv.side2 == river.side2 {
                    return true;
                } else if riv.side1 == river.side2 && riv.side2 == river.side1 {
                    return true;
                } else {
                    return false;
                }
            })
            .count();

        if found_rivs > 0 {
            return Err("Already foud such river.");
        }

        self.rivers.insert(self.current_free_id, river);
        self.graph.insert_node(
            self.current_free_id,
            found_hexes.keys().cloned().cloned().collect(),
        )?;
        self.current_free_id += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_map_insertions() {
        let mut map = Map::new();
        for r in -1..=1 {
            for q in -1..=1 {
                map.insert_hex(HexSite {
                    coord: HexCoordinates::new_axial(q, r),
                    kind: Field::Plain,
                })
                .unwrap();
            }
        }
        map.insert_river(RiverSite {
            side1: HexCoordinates::new_axial(0, -1),
            side2: HexCoordinates::new_axial(1, -1),
            kind: River::Stream,
        })
        .unwrap();

        map.insert_river(RiverSite {
            side1: HexCoordinates::new_axial(0, 0),
            side2: HexCoordinates::new_axial(1, -1),
            kind: River::Stream,
        })
        .unwrap();

        map.insert_river(RiverSite {
            side1: HexCoordinates::new_axial(0, 0),
            side2: HexCoordinates::new_axial(1, 0),
            kind: River::Stream,
        })
        .unwrap();

        map.insert_river(RiverSite {
            side1: HexCoordinates::new_axial(1, 0),
            side2: HexCoordinates::new_axial(0, 1),
            kind: River::Stream,
        })
        .unwrap();

        let mut graph = BidirectionalGraph::new();
        graph.insert_node(0, HashSet::default());
        graph.insert_node(1, [0].into_iter().cloned().collect());
        graph.insert_node(2, [1].into_iter().cloned().collect());
        graph.insert_node(3, [0, 1].into_iter().cloned().collect());
        graph.insert_node(4, [3, 1, 2].into_iter().cloned().collect());
        graph.insert_node(5, [4, 2].into_iter().cloned().collect());
        graph.insert_node(6, [3, 4].into_iter().cloned().collect());
        graph.insert_node(7, [6, 4, 5].into_iter().cloned().collect());
        graph.insert_node(8, [5, 7].into_iter().cloned().collect());
        graph.insert_node(9, [1, 2].into_iter().cloned().collect());
        graph.insert_node(10, [2, 4].into_iter().cloned().collect());
        graph.insert_node(11, [4, 5].into_iter().cloned().collect());
        graph.insert_node(12, [5, 7].into_iter().cloned().collect());

        assert_eq!(map.graph.graph, graph.graph);
    }

}
