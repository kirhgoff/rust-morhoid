use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

use rand::Rng;

pub type GenomeId = u64;
pub type Gene = usize;

pub const GENOME_LENGTH: usize = 64;
pub const GENE_COUNT: usize = 64;

pub const DEFILE: Gene = 25;
pub const SENSE: Gene = 26;
pub const TURN: Gene = 27;
pub const MOVE: Gene = 28;
pub const ATTACK: Gene = 29;
pub const REPRODUCE: Gene = 30;
pub const PHOTOSYNTHESIS: Gene = 31;

const KNOWN_GENES: [Gene; 7] = [DEFILE, SENSE, TURN, MOVE, ATTACK, REPRODUCE, PHOTOSYNTHESIS];
const RANDOM_PROGRAM_LENGTH: usize = GENOME_LENGTH - 4;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Genome {
    pub id: GenomeId,
    pub genes: [Gene; GENOME_LENGTH],
}

impl Genome {
    fn new_id() -> GenomeId {
        NEXT_ID.fetch_add(1, Ordering::SeqCst) as GenomeId
    }

    pub fn new_plant() -> Genome {
        Genome {
            id: Genome::new_id(),
            genes: [PHOTOSYNTHESIS; GENOME_LENGTH],
        }
    }

    pub fn new_predator() -> Genome {
        Genome {
            id: Genome::new_id(),
            genes: [ATTACK; GENOME_LENGTH],
        }
    }

    pub fn new_yeast() -> Genome {
        Genome {
            id: Genome::new_id(),
            genes: [REPRODUCE; GENOME_LENGTH],
        }
    }

    pub fn new_defiler() -> Genome {
        Genome {
            id: Genome::new_id(),
            genes: [DEFILE; GENOME_LENGTH],
        }
    }

    pub fn mutate(&mut self, index: usize, new_value: Gene) {
        self.genes[index] = new_value;
    }

    pub fn random(rng: &mut impl Rng) -> Genome {
        let mut genome = Genome::new_plant();
        let mut i = 0;
        while i < RANDOM_PROGRAM_LENGTH {
            let gene = KNOWN_GENES[rng.gen_range(0..KNOWN_GENES.len())];
            genome.mutate(i, gene);
            i += 1;

            let argument_count = match gene {
                SENSE => 2,
                TURN => 1,
                _ => 0,
            };
            for _ in 0..argument_count {
                genome.mutate(i, rng.gen_range(0..GENOME_LENGTH));
                i += 1;
            }
        }
        genome
    }

    pub fn offspring(&self, mutation_probability: f64, rng: &mut impl Rng) -> Genome {
        let mut child = Genome {
            id: Genome::new_id(),
            genes: self.genes,
        };
        if rng.gen_bool(mutation_probability) {
            child.mutate(
                rng.gen_range(0..GENOME_LENGTH),
                rng.gen_range(0..GENE_COUNT),
            );
        }
        child
    }
}

impl fmt::Debug for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Genome genes: {}",
            self.genes.map(|g| g.to_string()).join(" ")
        )
    }
}

impl PartialEq for Genome {
    fn eq(&self, other: &Self) -> bool {
        self.genes == other.genes
    }
}

pub struct GenomeDesc {
    pub reproduces: usize,
    pub attacks: usize,
    pub photosynthesis: usize,
    pub defiles: usize,
}

impl GenomeDesc {
    pub fn of(genome: &Genome) -> GenomeDesc {
        let mut reproduces: usize = 0;
        let mut attacks: usize = 0;
        let mut photosynthesis: usize = 0;
        let mut defiles: usize = 0;

        for gene in genome.genes.iter() {
            match *gene {
                ATTACK => attacks += 1,
                REPRODUCE => reproduces += 1,
                PHOTOSYNTHESIS => photosynthesis += 1,
                DEFILE => defiles += 1,
                _ => {}
            }
        }

        GenomeDesc {
            reproduces,
            attacks,
            photosynthesis,
            defiles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

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
        assert_ne!(genome1.id, genome2.id);
        assert_eq!(
            "Genome genes: 31 31 31",
            format!("{:?}", genome1).split_at(22).0
        );
    }

    #[test]
    fn offspring_gets_new_id() {
        let genome1 = Genome::new_plant();
        let genome2 = genome1.offspring(0.0, &mut StdRng::seed_from_u64(1));
        assert_ne!(genome1.id, genome2.id);
        assert_eq!(genome1, genome2);
    }

    #[test]
    fn mutate() {
        let genome1 = Genome::new_plant();
        let mut genome2 = genome1.offspring(0.0, &mut StdRng::seed_from_u64(1));
        assert_eq!(genome1, genome2);
        genome2.mutate(0, REPRODUCE);
        assert_ne!(genome1, genome2);
    }

    #[test]
    fn random_genome_is_valid() {
        let genome = Genome::random(&mut StdRng::seed_from_u64(1));
        assert!(genome.genes.iter().all(|&gene| gene < GENE_COUNT));
        assert_eq!(PHOTOSYNTHESIS, genome.genes[GENOME_LENGTH - 1]);
    }

    #[test]
    fn desc_of_counts_genes() {
        let desc = GenomeDesc::of(&Genome::new_plant());
        assert_eq!(GENOME_LENGTH, desc.photosynthesis);
        assert_eq!(0, desc.attacks);

        let desc2 = GenomeDesc::of(&Genome::new_predator());
        assert_eq!(0, desc2.photosynthesis);
        assert_eq!(GENOME_LENGTH, desc2.attacks);
    }
}
