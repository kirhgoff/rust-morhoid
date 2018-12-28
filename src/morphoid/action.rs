use morphoid::world::*;
use morphoid::entity::Entity;
use morphoid::cell_state::HealthType;

pub trait Action {
    // do something with stats or replace with dirt
    fn execute(&self, affector: &mut Affector);
}

// ---------------------------------

pub struct KillAction {
    x: Coords,
    y: Coords
}

impl KillAction {
    pub fn new(x: Coords, y: Coords) -> KillAction {
        KillAction { x, y }
    }
}

impl Action for KillAction {
    fn execute(&self, affector: &mut Affector) {
        affector.set_entity(self.x, self.y, Entity::Corpse(10), None, None);
    }
}

// ---------------------------------

pub struct UpdateHealthAction {
    pub x: Coords,
    pub y: Coords,
    pub health_delta: HealthType
}

impl UpdateHealthAction {
    pub fn new(x: Coords, y: Coords, health_delta: HealthType) -> UpdateHealthAction {
        UpdateHealthAction { x, y, health_delta }
    }
}

impl Action for UpdateHealthAction {
    fn execute(&self, affector: &mut Affector) {
        affector.update_health(self.x, self.y, self.health_delta);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use morphoid::genome::Genome;
    use morphoid::cell_state::CellState;
    use std::collections::LinkedList;
    use morphoid::processor::Processor;

    #[test]
    fn update_health_action_works() {
        let mut world = World::new(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState {health: 10}));
        match world.get_state(hash) {
            CellState {health} => assert_eq!(*health, 10),
            _ => panic!()
        }

        let update_heath_action = UpdateHealthAction { x:0, y:0, health_delta: 5};
        let mut list:Vec<Box<Action>> = Vec::new();
        list.push(Box::new(update_heath_action));

        Processor::new().apply(&list, &mut world);

        match world.get_state(hash) {
            CellState {health} => assert_eq!(*health, 15),
            _ => panic!()
        }
    }
}