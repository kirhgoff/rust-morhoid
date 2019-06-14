use std::collections::HashMap;
use crate::types::*;

impl GenomeStorage {
    pub fn new() -> GenomeStorage {
        GenomeStorage {
            genomes: HashMap::new(),
            descriptors: HashMap::new()
        }
    }

    pub fn put(&mut self, genome:Genome) -> GenomeId {
        let id = genome.id();

        self.descriptors.insert(id, GenomeDesc::build_from(&genome));
        self.genomes.insert(id, genome);

        id
    }

    pub fn remove(&mut self, id: GenomeId) {
        self.genomes.remove(&id);
        self.descriptors.remove(&id);
    }

    pub fn get(&self, id: GenomeId) -> Option<&Genome> {
        self.genomes.get(&id)
    }

    pub fn describe(&self, id: GenomeId) -> Option<&GenomeDesc> {
        self.descriptors.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_and_put() {
        let mut storage = GenomeStorage::new();
        let genome = Genome::new_plant();
        let old_id = genome.id();

        let new_id = storage.put(genome);
        assert_ne!(new_id, 0);
        assert_eq!(old_id, new_id);

        let found_genome = storage.get(new_id).unwrap();
        assert_eq!(new_id, found_genome.id());
    }

    #[test]
    fn test_describe() {
        let mut storage = GenomeStorage::new();
        let id = storage.put(Genome::new_plant());

        let desc = storage.describe(id).unwrap();
        assert_eq!(0, desc.attacks);
        assert_eq!(GENOME_LENGTH, desc.photosynthesis);
    }
}
