use morphoid::world::*;
use morphoid::entity::Entity;
use morphoid::cell_state::HealthType;

pub struct Action {
    pub execute: Box<Fn(&mut Affector)-> !>,
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
        affector.set_entity(self.x, self.y, Entity::Corpse(10), None);
    }
}

// ---------------------------------

pub struct UpdateHealthAction {
    pub x: Coords,
    pub y: Coords,
    pub health_delta: HealthType
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
        world.set_entity(0, 0, Entity::Cell(hash), Some(CellState {health: 10}));
        match world.get_state(hash) {
            CellState {health} => assert_eq!(*health, 10),
            _ => panic!()
        }

        let update_heath_action = UpdateHealthAction { x:0, y:0, health_delta: 5};
        let mut list = LinkedList::new();
        list.push_back(Box::new(update_heath_action));
        Processor::apply(&list, &mut world);

        match world.get_state(hash) {
            CellState {health} => assert_eq!(*health, 15),
            _ => panic!()
        }
    }
}