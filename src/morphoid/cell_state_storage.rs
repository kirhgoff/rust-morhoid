use morphoid::genome::HashType;
use morphoid::cell_state::CellState;
use std::collections::HashMap;

pub struct CellStateStorage {
    states: HashMap<HashType,CellState>
}

impl CellStateStorage {
    pub fn new() -> CellStateStorage {
        CellStateStorage {states: HashMap::new()}
    }

    pub fn put(&mut self, hash: HashType, cell_state: CellState) {
        self.states.insert(hash, cell_state);
    }

    fn get(&mut self, hash:HashType) -> Option<&mut CellState> {
        self.states.get_mut(&hash)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_read_and_write_cell_states() {
        let mut storage = CellStateStorage::new();
        let cell_state = CellState { health: 10 };
        let hash: HashType = 1;
        storage.put(hash, cell_state);
        {
            let mut state = storage.get(hash).unwrap();
            assert_eq!(state.health, 10);
            state.health -= 5;
        }


        let mut new_state = storage.get(hash).unwrap();
        assert_eq!(new_state.health, 5);
    }
}