use crate::types::*;

impl GenomeDesc {
    pub fn build_from(genome: &Genome) -> GenomeDesc {
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
            reproduces: reproduces,
            attacks: attacks,
            photosynthesis: photosynthesis,
            defiles: defiles
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_from() {
        let desc = GenomeDesc::build_from(&Genome::new_plant());
        assert_eq!(GENOME_LENGTH, desc.photosynthesis);
        assert_eq!(0, desc.attacks);

        let desc2 = GenomeDesc::build_from(&Genome::new_predator());
        assert_eq!(0, desc2.photosynthesis);
        assert_eq!(GENOME_LENGTH, desc2.attacks);
    }
}
