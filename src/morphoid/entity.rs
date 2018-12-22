
use morphoid::genome::HashType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(HashType),
    Corpse(i64)
}
