extern crate log;
extern crate sfml;

use super::graph::*;
use super::hexagons::*;
use super::types::*;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
pub struct HexSite {
    coord: HexCoordinates,
    kind: Field,
}

impl HexSite {
    pub fn new(coord: HexCoordinates, kind: Field) -> HexSite {
        HexSite { coord, kind }
    }

    pub fn coord(&self) -> &HexCoordinates {
        &self.coord
    }

    pub fn kind(&self) -> Field {
        self.kind
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RiverSite {
    side1: HexCoordinates,
    side2: HexCoordinates,
    kind: River,
}

impl RiverSite {
    pub fn new(
        side1: HexCoordinates,
        side2: HexCoordinates,
        kind: River,
    ) -> Result<Self, &'static str> {
        if side1.neighbors().contains(&side2) {
            Ok(RiverSite { side1, side2, kind })
        } else {
            Err("Creating RiverSite with non neighboring sides.")
        }
    }

    pub fn sides(&self) -> (&HexCoordinates, &HexCoordinates) {
        (&self.side1, &self.side2)
    }

    pub fn kind(&self) -> River {
        self.kind
    }
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

    pub fn hexes(&self) -> &HashMap<i32, HexSite> {
        &self.hexes
    }

    pub fn rivers(&self) -> &HashMap<i32, RiverSite> {
        &self.rivers
    }

    pub fn insert_hex(&mut self, hex: HexSite) -> Result<&mut Self, &'static str> {
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
            .clone()
            .into_iter()
            .filter(|(id, h)| {
                if neighbours.contains(&h.coord) {
                    return true;
                } else {
                    return false;
                }
            })
            .collect();

        self.hexes.insert(self.current_free_id, hex);

        self.graph.insert_node(
            self.current_free_id,
            found_neighbors.keys().cloned().collect(),
        )?;
        self.current_free_id += 1;
        Ok(self)
    }

    pub fn insert_river(&mut self, river: RiverSite) -> Result<&mut Self, &'static str> {
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
        Ok(self)
    }

    pub fn create_test_map() -> Self {
        debug!("Creating test map.");
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
        map.insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(0, -1),
                HexCoordinates::new_axial(1, -1),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap()
        .insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(0, 0),
                HexCoordinates::new_axial(1, -1),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap()
        .insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(0, 0),
                HexCoordinates::new_axial(1, 0),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap()
        .insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(1, 0),
                HexCoordinates::new_axial(0, 1),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap();
        map
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
        map.insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(0, -1),
                HexCoordinates::new_axial(1, -1),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap()
        .insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(0, 0),
                HexCoordinates::new_axial(1, -1),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap()
        .insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(0, 0),
                HexCoordinates::new_axial(1, 0),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap()
        .insert_river(
            RiverSite::new(
                HexCoordinates::new_axial(1, 0),
                HexCoordinates::new_axial(0, 1),
                River::Stream,
            )
            .unwrap(),
        )
        .unwrap();

        let mut graph = BidirectionalGraph::new();
        graph
            .insert_node(0, HashSet::default())
            .unwrap()
            .insert_node(1, [0].iter().cloned().collect())
            .unwrap()
            .insert_node(2, [1].iter().cloned().collect())
            .unwrap()
            .insert_node(3, [0, 1].iter().cloned().collect())
            .unwrap()
            .insert_node(4, [3, 1, 2].iter().cloned().collect())
            .unwrap()
            .insert_node(5, [4, 2].iter().cloned().collect())
            .unwrap()
            .insert_node(6, [3, 4].iter().cloned().collect())
            .unwrap()
            .insert_node(7, [6, 4, 5].iter().cloned().collect())
            .unwrap()
            .insert_node(8, [5, 7].iter().cloned().collect())
            .unwrap()
            .insert_node(9, [1, 2].iter().cloned().collect())
            .unwrap()
            .insert_node(10, [2, 4].iter().cloned().collect())
            .unwrap()
            .insert_node(11, [4, 5].iter().cloned().collect())
            .unwrap()
            .insert_node(12, [5, 7].iter().cloned().collect())
            .unwrap();

        assert_eq!(map.graph, graph);
    }

}
