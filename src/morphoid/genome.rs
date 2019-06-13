extern crate itertools;

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use self::itertools::Itertools;
use crate::morphoid::types::*;


static HASH_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Genome {
    fn new_id() -> GenomeId {
        HASH_COUNTER.fetch_add(1, Ordering::SeqCst) as GenomeId
    }

    pub fn new_plant() -> Genome {
        Genome {id: Genome::new_id(), genes: [PHOTOSYNTHESYS; GENOME_LENGTH]}
    }

    pub fn new_reproducing_plant() -> Genome {
        let mut genes = [PHOTOSYNTHESYS; GENOME_LENGTH];
        for i in 0..GENOME_LENGTH {
            if i % 2 != 0 { genes[i] = REPRODUCE; }
        }
        Genome {id: Genome::new_id(), genes: genes}
    }

    pub fn new_predator() -> Genome {
        Genome {id: Genome::new_id(), genes: [ATTACK; GENOME_LENGTH]}
    }

    pub fn new_yeast() -> Genome {
        Genome {id: Genome::new_id(), genes: [REPRODUCE; GENOME_LENGTH]}
    }

    pub fn new_defiler() -> Genome {
        Genome {id: Genome::new_id(), genes: [DEFILE; GENOME_LENGTH]}
    }

    pub fn id(&self) -> GenomeId {
        self.id
    }

    pub fn mutate(&mut self, index: usize, new_value: Gene) {
        self.genes[index] = new_value;
    }

    pub fn clone(&self) -> Genome {
        let mut new_genome = Genome {id: Genome::new_id(), genes: [PHOTOSYNTHESYS; GENOME_LENGTH]};
        new_genome.genes.copy_from_slice(&self.genes[..]);
        new_genome
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
    fn partial_eq_impl() {
        let genome1 = Genome::new_plant();
        let genome2 = Genome::new_plant();
        let mut genome3 = Genome::new_plant();
        genome3.genes[0] = 22;

        assert_eq!(genome1, genome2);
        assert_ne!(genome2, genome3);
    }

    #[test]
    fn debug_impl() {
        let genome1 = Genome::new_plant();
        let genome2 = Genome::new_plant();
        assert_ne!(genome1.id(), genome2.id());
        assert_eq!("Genome genes: 31 31 31", format!("{:?}", genome1).split_at(22).0);
    }

    #[test]
    fn clone() {
        let genome1 = Genome::new_plant();
        let genome2 = genome1.clone();
        assert_ne!(genome1.id(), genome2.id());
        assert_eq!(genome1, genome2);
    }

    #[test]
    fn mutate() {
        let genome1 = Genome::new_plant();
        let mut genome2 = genome1.clone();
        assert_eq!(genome1, genome2);
        genome2.mutate(0, REPRODUCE);
        assert_ne!(genome1, genome2);
    }


}