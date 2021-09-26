use crate::instruction::Instruction;
use crate::types::{Itype, Rtype, Stype};

#[derive(Debug)]
pub enum DecodingError {
    Unsupported,
}

pub fn decode_instruction(inst: u32) -> Result<Instruction, DecodingError> {
    let opcode = inst & 0x7f;
    match opcode & 0b11 {
        0b11 => match opcode {
            0b0000011 => {
                // LOAD
                println!("unsupported opcode LOAD 0x{:08x}", inst);
                let _inst = Itype::from(inst);
                Err(DecodingError::Unsupported)
            }
            0b0000111 => {
                // LOAD-FP
                println!("unsupported opcode LOAD-FP 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b0001111 => {
                // MISC-MEM
                println!("unsupported opcode MISC-MEM 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b0010011 => {
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
            0b0010111 => {
                // AUIPC
                println!("unsupported opcode AUIPC 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b0011011 => {
                // OP-IMM-32
                println!("unsupported opcode OP-IMM-32 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b0100011 => {
                // STORE
                println!("unsupported opcode STORE 0x{:08x}", inst);
                let _inst = Stype::from(inst);
                Err(DecodingError::Unsupported)
            }
            0b0100111 => {
                // STORE-FP
                println!("unsupported opcode STORE-FP 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b0101111 => {
                // AMO
                println!("unsupported opcode AMO 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b0110011 => {
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
            0b0110111 => {
                // LUI
                println!("unsupported opcode LUI 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b0111011 => {
                // OP-32
                println!("unsupported opcode OP-32 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1000011 => {
                // MADD
                println!("unsupported opcode MADD 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1000111 => {
                // MSUB
                println!("unsupported opcode MSUB 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1001011 => {
                // NMSUB
                println!("unsupported opcode NMSUB 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1001111 => {
                // NMADD
                println!("unsupported opcode NMADD 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1010011 => {
                // OP-FP
                println!("unsupported opcode OP-FP 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1100011 => {
                // BRANCH
                println!("unsupported opcode BRANCH 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1100111 => {
                // JALR
                println!("unsupported opcode JALR 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1101111 => {
                // JAL
                println!("unsupported opcode JAL 0x{:08x}", inst);
                Err(DecodingError::Unsupported)
            }
            0b1110011 => {
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
