use crate::types::{Itype, Rtype, Utype};

pub enum Instruction {
    Add(Rtype),
    Addi(Itype),
    Auipc(Utype),
    Lw(Itype)
}
