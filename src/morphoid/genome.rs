
use std::collections::HashMap;
use std::fmt;

type HashType = u32;
type GeneType = u32;

struct Genome {
    genes: [GeneType; 64]
}

impl Genome {
    fn new_plant() -> Genome {
        Genome {genes: [31; 64]}
    }

    pub fn hash(&self) -> HashType {
        0
    }
}

impl fmt::Debug for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Genome genes: "); // TODO: process Result
        self.genes[..].fmt(f)
    }
}

impl PartialEq for Genome {
    fn eq(&self, other: &Self) -> bool {
        //self.genes == other.genes //TODO: why it does not work?
        self.genes.iter()
            .zip(other.genes.iter())
            .find(|(a,b)| a != b) == None
    }
}

struct GenomeStorage {
    genomes: HashMap<HashType,Genome>
}

impl GenomeStorage {
    fn new() -> GenomeStorage {
        GenomeStorage {genomes: HashMap::new()}
    }

    fn put(&mut self, genome:Genome) -> HashType {
        let hash = genome.hash();
        self.genomes.insert(hash, genome);
        hash
    }

    fn get(&self, hash:HashType) -> Option<&Genome> {
        self.genomes.get(&hash)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genome_impl_partial_eq() {
        let genome1 = Genome::new_plant();
        let genome2 = Genome::new_plant();
        let mut genome3 = Genome::new_plant();
        genome3.genes[0] = 22;

        assert_eq!(genome1, genome2);
        assert_ne!(genome2, genome3);
    }

    #[test]
    fn genome_impl_debug() {
        let genome1 = Genome::new_plant();
        let genome2 = Genome::new_plant();
        assert_ne!(genome1.hash(), genome2.hash());
        assert_eq!("xxxxx", format!("{:?}", genome1));
    }

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