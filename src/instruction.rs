use crate::types::{Itype, Rtype};

pub enum Instruction {
    Add(Rtype),
    Addi(Itype),
}
