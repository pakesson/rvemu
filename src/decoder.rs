use crate::instruction::Instruction;
use crate::types::{Btype, Itype, Jtype, Rtype, Stype, Utype};

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
                    0b000 => Ok(Instruction::Lb(inst)),
                    0b001 => Ok(Instruction::Lh(inst)),
                    0b010 => Ok(Instruction::Lw(inst)),
                    0b100 => Ok(Instruction::Lbu(inst)),
                    0b101 => Ok(Instruction::Lhu(inst)),
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
                    0b010 => {
                        // SLTI
                        Ok(Instruction::Slti(inst))
                    }
                    0b011 => {
                        // SLTIU
                        Ok(Instruction::Sltiu(inst))
                    }
                    0b100 => {
                        // XORI
                        Ok(Instruction::Xori(inst))
                    }
                    0b110 => {
                        // ORI
                        Ok(Instruction::Ori(inst))
                    }
                    0b111 => {
                        // ANDI
                        Ok(Instruction::Andi(inst))
                    }
                    0b001 => {
                        // SLLI
                        Ok(Instruction::Slli(inst))
                    }
                    0b101 => match (inst.imm >> 10) & 0b1 {
                        0 => Ok(Instruction::Srli(inst)),
                        1 => Ok(Instruction::Srai(inst)),
                        _ => unreachable!(),
                    },
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
                let inst = Stype::from(inst);
                match inst.funct3 & 0b111 {
                    0b000 => Ok(Instruction::Sb(inst)),
                    0b001 => Ok(Instruction::Sh(inst)),
                    0b010 => Ok(Instruction::Sw(inst)),
                    _ => Err(DecodingError::Unsupported),
                }
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
                    0b000 => match (inst.funct7 >> 6) & 0b1 {
                        0 => Ok(Instruction::Add(inst)),
                        1 => Ok(Instruction::Sub(inst)),
                        _ => unreachable!(),
                    },
                    0b001 => Ok(Instruction::Sll(inst)),
                    0b010 => Ok(Instruction::Slt(inst)),
                    0b011 => Ok(Instruction::Sltu(inst)),
                    0b100 => Ok(Instruction::Xor(inst)),
                    0b101 => match (inst.funct7 >> 6) & 0b1 {
                        0 => Ok(Instruction::Srl(inst)),
                        1 => Ok(Instruction::Sra(inst)),
                        _ => unreachable!(),
                    },
                    0b110 => Ok(Instruction::Or(inst)),
                    0b111 => Ok(Instruction::And(inst)),
                    _ => {
                        println!("Unsupported funct3 {:#02x} in OP inst", inst.funct3);
                        Err(DecodingError::Unsupported)
                    }
                }
            }
            0b01101 => {
                // LUI
                let inst = Utype::from(inst);
                Ok(Instruction::Lui(inst))
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
                let inst = Btype::from(inst);
                match inst.funct3 {
                    0b000 => Ok(Instruction::Beq(inst)),
                    0b001 => Ok(Instruction::Bne(inst)),
                    0b100 => Ok(Instruction::Blt(inst)),
                    0b101 => Ok(Instruction::Bge(inst)),
                    0b110 => Ok(Instruction::Bltu(inst)),
                    0b111 => Ok(Instruction::Bgeu(inst)),
                    _ => Err(DecodingError::Unsupported),
                }
            }
            0b11001 => {
                let inst = Itype::from(inst);
                match inst.funct3 {
                    0b000 => {
                        // JALR
                        Ok(Instruction::Jalr(inst))
                    }
                    _ => Err(DecodingError::Unsupported),
                }
            }
            0b11011 => {
                // JAL
                let inst = Jtype::from(inst);
                Ok(Instruction::Jal(inst))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(
            decode_instruction(0x00457593).unwrap(),
            Instruction::Andi(Itype::from(0x00457593))
        );
        assert_eq!(
            decode_instruction(0x00256613).unwrap(),
            Instruction::Ori(Itype::from(0x00256613))
        );
        assert_eq!(
            decode_instruction(0x00a54693).unwrap(),
            Instruction::Xori(Itype::from(0x00a54693))
        );
    }
}
