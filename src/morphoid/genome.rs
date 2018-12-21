extern crate itertools;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use self::itertools::Itertools;

type HashType = u64;
type GeneType = u32;

static HASH_COUNTER: AtomicUsize = AtomicUsize::new(0);
const GENE_LENGTH: usize = 64;

const PHOTOSYNTHESYS: GeneType = 31;

struct Genome {
    id: u64,
    genes: [GeneType; GENE_LENGTH]
}

impl Genome {
    fn new_plant() -> Genome {
        Genome {id: HASH_COUNTER.fetch_add(1, Ordering::SeqCst) as u64, genes: [PHOTOSYNTHESYS; GENE_LENGTH]}
    }

    pub fn hash(&self) -> HashType {
        self.id
    }
}

impl fmt::Debug for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Genome genes: {}", self.genes.iter().format(" "))
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
        assert_eq!("Genome genes: 31 31 31", format!("{:?}", genome1).split_at(22).0);
    }
}