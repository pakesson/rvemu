use crate::types::{Itype, Rtype, Utype};

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
