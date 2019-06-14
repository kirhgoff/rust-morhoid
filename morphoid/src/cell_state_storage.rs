use std::collections::HashMap;

use crate::types::*;

impl CellStateStorage {
    pub fn new() -> CellStateStorage {
        CellStateStorage {states: HashMap::new()}
    }

    pub fn put(&mut self, hash: GenomeId, cell_state: CellState) {
        self.states.insert(hash, cell_state);
    }

    pub fn get_mut(&mut self, hash: GenomeId) -> &mut CellState {
        self.states.get_mut(&hash).unwrap()
    }

    pub fn get(&self, hash: GenomeId) -> &CellState {
        self.states.get(&hash).unwrap()
    }

    pub fn remove(&mut self, hash: GenomeId) {
        self.states.remove(&hash);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_read_and_write_cell_states() {
        let mut storage = CellStateStorage::new();
        let cell_state = CellState::default();
        let hash: GenomeId = 1;
        storage.put(hash, cell_state);
        {
            let state = storage.get_mut(hash);
            assert_eq!(state.health, 10);
            state.health -= 5;
        }

        assert_eq!(storage.get(hash).health, 5);
    }
}
