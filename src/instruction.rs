use crate::types::{Itype, Rtype, Utype};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(Rtype),
    Addi(Itype),
    Auipc(Utype),
    Lw(Itype),
    Slti(Itype),
    Sltiu(Itype),
    Xori(Itype),
    Andi(Itype),
    Ori(Itype),
    Slli(Itype),
    Srli(Itype),
    Srai(Itype),
}
