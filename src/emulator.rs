use crate::decoder::decode_instruction;
use crate::instruction::Instruction;

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

    pub fn execute_instruction(&mut self, inst: Instruction) {
        match inst {
            Instruction::Add(inst) => {
                self.setreg(
                    inst.rd,
                    self.getreg(inst.rs1).wrapping_add(self.getreg(inst.rs2)),
                );
            }
            Instruction::Addi(inst) => {
                self.setreg(
                    inst.rd,
                    self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64),
                );
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.pc >= self.memory.len() as u64 {
                break;
            }
            let inst = self.fetch_instruction();
            let decoded_inst = decode_instruction(inst).unwrap();
            self.pc += 4;
            self.execute_instruction(decoded_inst);
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
}
