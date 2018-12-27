use std::collections::HashMap;
use morphoid::genome::*;

pub struct GenomeStorage {
    genomes: HashMap<HashType,Genome>
}

impl GenomeStorage {
    pub fn new() -> GenomeStorage {
        GenomeStorage {genomes: HashMap::new()}
    }

    fn put(&mut self, genome:Genome) -> HashType {
        let hash = genome.hash();
        self.genomes.insert(hash, genome);
        hash
    }

    pub fn get(&self, hash:HashType) -> Option<&Genome> {
        self.genomes.get(&hash)
    }

    pub fn remove(&mut self, hash:HashType) {
        self.genomes.remove(&hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_read_and_write_genomes() {
        let mut storage = GenomeStorage::new();
        let genome = Genome::new_plant();
        let genome_hash = genome.hash();

        let hash = storage.put(genome);
        assert_ne!(hash, 0);
        assert_eq!(genome_hash, hash);

        let found_genome = storage.get(hash).unwrap();
        assert_eq!(hash, found_genome.hash());
        //assert_eq!(*found_genome, genome); // TODO: what about moving?
    }
}