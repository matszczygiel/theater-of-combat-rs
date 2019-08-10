use std::collections::HashMap;

use super::unit::*;

use crate::systems::*;

pub struct UnitSet {
    units: HashMap<i32, Unit>,
    current_free_id: i32,
}

impl UnitSet {
    pub fn new() -> Self {
        UnitSet {
            units: HashMap::new(),
            current_free_id: 0,
        }
    }

    pub fn push_unit(
        &mut self,
        systems: &mut GameSystems,
        kind: UnitType,
        name: String,
    ) -> Result<i32, &'static str> {
        let unit = Unit::new(name, kind, self.current_free_id);
        self.current_free_id += 1;
        self.units
            .insert(unit.id(), unit.clone())
            .ok_or("Unit set already contains unit with this id.");

        UnitSet::register_unit_in_game_systems(systems, &unit)?;

        Ok(unit.id())
    }

    fn register_unit_in_game_systems(
        systems: &mut GameSystems,
        unit: &Unit,
    ) -> Result<(), &'static str> {
        match unit.kind() {
            UnitType::Mechanized => {
                systems
                    .moving
                    .components
                    .insert(unit.id(), components::MovingComponent::new(unit.id(), 15))
                    .ok_or("Moving system already registered unit with this id.");
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn create_test_unit_set(systems: &mut GameSystems) -> Self {
        let mut set = Self::new();
        set.push_unit(systems, UnitType::Mechanized, String::from("test unit 0")).unwrap();
        set.push_unit(systems, UnitType::Mechanized, String::from("test unit 1")).unwrap();
        set.push_unit(systems, UnitType::Mechanized, String::from("test unit 2")).unwrap();
        set
    }
}

#[cfg(test)]
mod tests {
}
