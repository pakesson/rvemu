use crate::instruction::Instruction;
use crate::types::{Itype, Rtype, Stype, Utype};

#[derive(Debug)]
pub enum DecodingError {
    Unsupported,
}

pub fn decode_instruction(inst: u32) -> Result<Instruction, DecodingError> {
    let opcode = inst & 0x7f;
    match opcode & 0b11 {
        0b11 => match opcode >> 2 {
            0b00000 => {
                // LOAD
                let inst = Itype::from(inst);
                match inst.funct3 & 0b111 {
                    0b010 => Ok(Instruction::Lw(inst)),
                    _ => Err(DecodingError::Unsupported),
                }
            }
            0b00001 => {
                // LOAD-FP
                println!("unsupported opcode LOAD-FP 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b00011 => {
                // MISC-MEM
                println!("unsupported opcode MISC-MEM 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b00100 => {
                // OP-IMM
                let inst = Itype::from(inst);
                match inst.funct3 {
                    0b000 => {
                        // ADDI
                        Ok(Instruction::Addi(inst))
                    }
                    _ => {
                        println!("Unsupported funct3 {:#02x} in OP-IMM inst", inst.funct3);
                        Err(DecodingError::Unsupported)
                    }
                }
            }
            0b00101 => {
                // AUIPC
                let inst = Utype::from(inst);
                Ok(Instruction::Auipc(inst))
            }
            0b00110 => {
                // OP-IMM-32
                println!("unsupported opcode OP-IMM-32 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b01000 => {
                // STORE
                println!("unsupported opcode STORE 0x{:08x}", inst);
                let _inst = Stype::from(inst);
                Err(DecodingError::Unsupported)
            }
            0b01001 => {
                // STORE-FP
                println!("unsupported opcode STORE-FP 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b01011 => {
                // AMO
                println!("unsupported opcode AMO 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b01100 => {
                // OP
                let inst = Rtype::from(inst);
                match inst.funct3 {
                    0b000 => {
                        // ADD
                        Ok(Instruction::Add(inst))
                    }
                    _ => {
                        println!("Unsupported funct3 {:#02x} in OP inst", inst.funct3);
                        Err(DecodingError::Unsupported)
                    }
                }
            }
            0b01101 => {
                // LUI
                println!("unsupported opcode LUI 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b01110 => {
                // OP-32
                println!("unsupported opcode OP-32 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b10000 => {
                // MADD
                println!("unsupported opcode MADD 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b10001 => {
                // MSUB
                println!("unsupported opcode MSUB 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b10010 => {
                // NMSUB
                println!("unsupported opcode NMSUB 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b10011 => {
                // NMADD
                println!("unsupported opcode NMADD 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b10100 => {
                // OP-FP
                println!("unsupported opcode OP-FP 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b11000 => {
                // BRANCH
                println!("unsupported opcode BRANCH 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b11001 => {
                // JALR
                println!("unsupported opcode JALR 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b11011 => {
                // JAL
                println!("unsupported opcode JAL 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b11100 => {
                // SYSTEM
                println!("unsupported opcode SYSTEM 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            _ => {
                println!("unsupported instruction 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
        },
        _ => Err(DecodingError::Unsupported), // Compressed instruction
    }
}
