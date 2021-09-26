#[derive(Debug, PartialEq)]
pub struct Rtype {
    pub rd: usize,
    pub funct3: u32,
    pub rs1: usize,
    pub rs2: usize,
    pub funct7: u32,
}

impl From<u32> for Rtype {
    fn from(inst: u32) -> Self {
        Self {
            rd: ((inst >> 7) & 0x1f) as usize,
            funct3: ((inst >> 12) & 0x7),
            rs1: ((inst >> 15) & 0x1f) as usize,
            rs2: ((inst >> 20) & 0x1f) as usize,
            funct7: ((inst >> 25) & 0x7f),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Itype {
    pub rd: usize,
    pub funct3: u32,
    pub rs1: usize,
    pub imm: i32,
}

impl From<u32> for Itype {
    fn from(inst: u32) -> Self {
        Self {
            rd: ((inst >> 7) & 0x1f) as usize,
            funct3: ((inst >> 12) & 0x7),
            rs1: ((inst >> 15) & 0x1f) as usize,
            imm: (inst as i32 as i64 >> 20) as i32,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Stype {
    pub imm: i32,
    pub funct3: u32,
    pub rs1: usize,
    pub rs2: usize,
}

impl From<u32> for Stype {
    fn from(inst: u32) -> Self {
        Self {
            imm: ((inst >> 7) & 0x1f) as i32 | ((inst & 0xfe000000) as i32 >> 20),
            funct3: ((inst >> 12) & 0x7),
            rs1: ((inst >> 15) & 0x1f) as usize,
            rs2: ((inst >> 20) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Utype {
    pub rd: usize,
    pub imm: i32,
}

impl From<u32> for Utype {
    fn from(inst: u32) -> Self {
        Self {
            rd: ((inst >> 7) & 0x1f) as usize,
            imm: (inst & 0xfffff000) as i32,
        }
    }
}

// TODO: Add Btype and Jtype instructions

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_itype() {
        let inst = Itype::from(0x80152583); // lw	a1,-2047(a0)
        assert_eq!(inst.rs1, 10); // base
        assert_eq!(inst.rd, 11); // dst
        assert_eq!(inst.funct3, 0b010); // width
        assert_eq!(inst.imm, -2047); // offset
    }

    #[test]
    fn test_decode_stype() {
        let inst = Stype::from(0x80b520a3); // sw	a1,-2047(a0)
        assert_eq!(inst.rs1, 10); // base
        assert_eq!(inst.rs2, 11); // src
        assert_eq!(inst.funct3, 0b010); // width
        assert_eq!(inst.imm, -2047); // offset

        let inst = Stype::from(0x7eb52fa3); // sw	a1,2047(a0)
        assert_eq!(inst.rs1, 10); // base
        assert_eq!(inst.rs2, 11); // src
        assert_eq!(inst.funct3, 0b010); // width
        assert_eq!(inst.imm, 2047); // offset
    }
}
