
use morphoid::genome::GenomeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(GenomeId),
    Corpse(i64)
}
