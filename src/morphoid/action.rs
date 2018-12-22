use morphoid::world::*;
use morphoid::entity::Entity;

pub trait Action : Sized {
    // do something with stats or replace with dirt
    fn execute(&self, affector: &mut Affector);
}

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
        affector.set_entity(self.x, self.y, Entity::Corpse(10));
    }
}

