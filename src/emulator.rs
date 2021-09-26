pub struct Rtype {
    rd: usize,
    funct3: u32,
    rs1: usize,
    rs2: usize,
    funct7: u32,
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

pub struct Itype {
    rd: usize,
    funct3: u32,
    rs1: usize,
    imm: i32,
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

pub struct Stype {
    imm: i32,
    funct3: u32,
    rs1: usize,
    rs2: usize,
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

pub struct Utype {
    rd: usize,
    imm: i32,
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

pub struct Emulator {
    pub regs: [u64; 32],
    pub pc: u64,
    pub memory: Vec<u8>,
}

impl Emulator {
    pub fn new(memory: Vec<u8>) -> Self {
        Self {
            regs: [0; 32],
            pc: 0,
            memory: memory,
        }
    }

    pub fn getreg(&self, reg: usize) -> u64 {
        if reg == 0 {
            0
        } else {
            self.regs[reg]
        }
    }

    pub fn setreg(&mut self, reg: usize, val: u64) {
        if reg != 0 {
            self.regs[reg] = val;
        }
    }

    pub fn fetch_instruction(&self) -> u32 {
        self.memory[self.pc as usize] as u32
            | (self.memory[(self.pc + 1) as usize] as u32) << 8
            | (self.memory[(self.pc + 2) as usize] as u32) << 16
            | (self.memory[(self.pc + 3) as usize] as u32) << 24
    }

    pub fn execute_instruction(&mut self, inst: u32) {
        let opcode = inst & 0x7f;
        if (opcode >> 2) & 0x7 == 0x7 {
            println!("unsupported instruction size (>32b)");
        }
        match opcode {
            0b0000011 => {
                // LOAD
                println!("unsupported opcode LOAD 0x{:08x}", inst);
                let _inst = Itype::from(inst);
            },
            0b0000111 => {
                // LOAD-FP
                println!("unsupported opcode LOAD-FP 0x{:08x}", inst);
            },
            0b0001111 => {
                // MISC-MEM
                println!("unsupported opcode MISC-MEM 0x{:08x}", inst);
            },
            0b0010011 => {
                // OP-IMM
                let inst = Itype::from(inst);
                match inst.funct3 {
                    0b000 => {
                        // ADDI
                        self.setreg(inst.rd, self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64));
                    }
                    _ => {
                        println!("Unsupported funct3 {:#02x} in OP-IMM inst", inst.funct3);
                    }
                }

            },
            0b0010111 => {
                // AUIPC
                println!("unsupported opcode AUIPC 0x{:08x}", inst);
            },
            0b0011011 => {
                // OP-IMM-32
                println!("unsupported opcode OP-IMM-32 0x{:08x}", inst);
            },
            0b0100011 => {
                // STORE
                println!("unsupported opcode STORE 0x{:08x}", inst);
                let _inst = Stype::from(inst);
            },
            0b0100111 => {
                // STORE-FP
                println!("unsupported opcode STORE-FP 0x{:08x}", inst);
            },
            0b0101111 => {
                // AMO
                println!("unsupported opcode AMO 0x{:08x}", inst);
            },
            0b0110011 => {
                // OP
                let inst = Rtype::from(inst);
                match inst.funct3 {
                    0b000 => {
                        // ADD
                        self.setreg(inst.rd, self.getreg(inst.rs1).wrapping_add(self.getreg(inst.rs2)));
                    }
                    _ => {
                        println!("Unsupported funct3 {:#02x} in OP inst", inst.funct3);
                    }
                }
            },
            0b0110111 => {
                // LUI
                println!("unsupported opcode LUI 0x{:08x}", inst);
            },
            0b0111011 => {
                // OP-32
                println!("unsupported opcode OP-32 0x{:08x}", inst);
            },
            0b1000011 => {
                // MADD
                println!("unsupported opcode MADD 0x{:08x}", inst);
            },
            0b1000111 => {
                // MSUB
                println!("unsupported opcode MSUB 0x{:08x}", inst);
            },
            0b1001011 => {
                // NMADD
                println!("unsupported opcode NMSUB 0x{:08x}", inst);
            },
            0b1001111 => {
                // NMADD
                println!("unsupported opcode NMADD 0x{:08x}", inst);
            },
            0b1010011 => {
                // OP-FP
                println!("unsupported opcode OP-FP 0x{:08x}", inst);
            },
            0b1100011 => {
                // BRANCH
                println!("unsupported opcode BRANCH 0x{:08x}", inst);
            },
            0b1100111 => {
                // JALR
                println!("unsupported opcode JALR 0x{:08x}", inst);
            },
            0b1101111 => {
                // JAL
                println!("unsupported opcode JAL 0x{:08x}", inst);
            },
            0b1110011 => {
                // SYSTEM
                println!("unsupported opcode SYSTEM 0x{:08x}", inst);
            },
            _ => {
                println!("unsupported instruction 0x{:08x}", inst);
            },
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.pc >= self.memory.len() as u64 {
                break;
            }
            let inst = self.fetch_instruction();
            self.pc += 4;
            self.execute_instruction(inst);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_addi_add() {
        let code = vec![
            0x13, 0x05, 0x60, 0x00, // addi a0,x0,6 (li a0,6)
            0x93, 0x05, 0x40, 0x00, // addi a1,x0,4 (li a1,4)
            0x33, 0x05, 0xb5, 0x00, // add a0,a0,a1
        ];

        let mut emu = Emulator::new(code);
        emu.run();

        assert_eq!(emu.getreg(10), 0x0a);
    }

    #[test]
    fn test_auipc_lw() {
        let code = vec![
            0x13, 0x05, 0x10, 0x00, // li	  a0,1
            0x97, 0x05, 0x00, 0x00, // auipc  a1,0x0
            0x83, 0xa5, 0x05, 0x00, // lw     a1,0(a1)
        ];

        let mut emu = Emulator::new(code);
        emu.run();

        assert_eq!(emu.getreg(11), 0x00100513);
    }

    #[test]
    fn test_decode_itype() {
        let inst = Itype::from(0x80152583); // lw	a1,-2047(a0)
        assert_eq!(inst.rs1, 10);       // base
        assert_eq!(inst.rd, 11);        // dst
        assert_eq!(inst.funct3, 0b010); // width
        assert_eq!(inst.imm, -2047);    // offset
    }

    #[test]
    fn test_decode_stype() {
        let inst = Stype::from(0x80b520a3); // sw	a1,-2047(a0)
        assert_eq!(inst.rs1, 10);       // base
        assert_eq!(inst.rs2, 11);       // src
        assert_eq!(inst.funct3, 0b010); // width
        assert_eq!(inst.imm, -2047);    // offset

        let inst = Stype::from(0x7eb52fa3); // sw	a1,2047(a0)
        assert_eq!(inst.rs1, 10);       // base
        assert_eq!(inst.rs2, 11);       // src
        assert_eq!(inst.funct3, 0b010); // width
        assert_eq!(inst.imm, 2047);    // offset
    }
}