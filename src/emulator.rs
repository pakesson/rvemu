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
            Instruction::And(inst) => {
                self.setreg(inst.rd, self.getreg(inst.rs1) & self.getreg(inst.rs2))
            }
            Instruction::Andi(inst) => {
                self.setreg(inst.rd, (inst.imm as i64 as u64) & self.getreg(inst.rs1))
            }
            Instruction::Auipc(inst) => {
                self.setreg(
                    inst.rd,
                    // PC has already been increased
                    self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64),
                );
            }
            Instruction::Beq(inst) => {
                if self.getreg(inst.rs1) == self.getreg(inst.rs2) {
                    // PC has already been increased
                    self.pc = self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64);
                }
            }
            Instruction::Bne(inst) => {
                if self.getreg(inst.rs1) != self.getreg(inst.rs2) {
                    // PC has already been increased
                    self.pc = self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64);
                }
            }
            Instruction::Blt(inst) => {
                if (self.getreg(inst.rs1) as i64) < (self.getreg(inst.rs2) as i64) {
                    // PC has already been increased
                    self.pc = self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64);
                }
            }
            Instruction::Bge(inst) => {
                if (self.getreg(inst.rs1) as i64) > (self.getreg(inst.rs2) as i64) {
                    // PC has already been increased
                    self.pc = self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64);
                }
            }
            Instruction::Bltu(inst) => {
                if self.getreg(inst.rs1) < self.getreg(inst.rs2) {
                    // PC has already been increased
                    self.pc = self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64);
                }
            }
            Instruction::Bgeu(inst) => {
                if self.getreg(inst.rs1) > self.getreg(inst.rs2) {
                    // PC has already been increased
                    self.pc = self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64);
                }
            }
            Instruction::Jal(inst) => {
                // TODO: instruction-address-misaligned exception if address is not aligned
                // PC has already been increased
                self.setreg(inst.rd, self.pc);
                self.pc = self.pc.wrapping_sub(4).wrapping_add(inst.imm as i64 as u64);
            }
            Instruction::Jalr(inst) => {
                // TODO: instruction-address-misaligned exception if address is not aligned
                // PC has already been increased
                self.setreg(inst.rd, self.pc);
                self.pc = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64);
            }
            Instruction::Lui(inst) => {
                self.setreg(inst.rd, inst.imm as i64 as u64);
            }
            Instruction::Lb(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                let value = self.memory[address] as i8 as i64 as u64;
                self.setreg(inst.rd, value);
            }
            Instruction::Lbu(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                let value = self.memory[address] as u64;
                self.setreg(inst.rd, value);
            }
            Instruction::Lh(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                let value: [u8; 2] = [self.memory[address], self.memory[address + 1]];
                self.setreg(inst.rd, i16::from_le_bytes(value) as i64 as u64);
            }
            Instruction::Lhu(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                let value = self.memory[address] as u64 | ((self.memory[address + 1] as u64) << 8);
                self.setreg(inst.rd, value);
            }
            Instruction::Lw(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                let value = self.memory[address] as u64
                    | ((self.memory[address + 1] as u64) << 8)
                    | ((self.memory[address + 2] as u64) << 16)
                    | ((self.memory[address + 3] as u64) << 24);
                self.setreg(inst.rd, value);
            }
            Instruction::Or(inst) => {
                self.setreg(inst.rd, self.getreg(inst.rs1) | self.getreg(inst.rs2))
            }
            Instruction::Ori(inst) => {
                self.setreg(inst.rd, (inst.imm as i64 as u64) | self.getreg(inst.rs1))
            }
            Instruction::Sb(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                self.memory[address] = self.getreg(inst.rs2) as u8;
            }
            Instruction::Sh(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                let value = self.getreg(inst.rs2);
                self.memory[address] = value as u8;
                self.memory[address + 1] = (value >> 8) as u8;
            }
            Instruction::Sll(inst) => {
                let shamt = self.getreg(inst.rs2) & 0b11111;
                self.setreg(inst.rd, self.getreg(inst.rs1) << shamt);
            }
            Instruction::Slli(inst) => {
                let shamt = inst.imm & 0b111111; // Shift amount
                self.setreg(inst.rd, self.getreg(inst.rs1) << shamt);
            }
            Instruction::Slt(inst) => {
                self.setreg(
                    inst.rd,
                    ((self.getreg(inst.rs1) as i64) < (self.getreg(inst.rs2) as i64)) as u64,
                );
            }
            Instruction::Slti(inst) => {
                self.setreg(
                    inst.rd,
                    ((self.getreg(inst.rs1) as i64) < (inst.imm as i64)) as u64,
                );
            }
            Instruction::Sltiu(inst) => {
                self.setreg(
                    inst.rd,
                    (self.getreg(inst.rs1) < (inst.imm as i64 as u64)) as u64,
                );
            }
            Instruction::Sltu(inst) => {
                self.setreg(
                    inst.rd,
                    (self.getreg(inst.rs1) < self.getreg(inst.rs2)) as u64,
                );
            }
            Instruction::Sra(inst) => {
                let shamt = self.getreg(inst.rs2) & 0b11111; // Shift amount
                self.setreg(inst.rd, ((self.getreg(inst.rs1) as i64) >> shamt) as u64);
            }
            Instruction::Srai(inst) => {
                let shamt = inst.imm & 0b111111; // Shift amount
                self.setreg(inst.rd, ((self.getreg(inst.rs1) as i64) >> shamt) as u64);
            }
            Instruction::Srl(inst) => {
                let shamt = self.getreg(inst.rs2) & 0b11111; // Shift amount
                self.setreg(inst.rd, self.getreg(inst.rs1) >> shamt);
            }
            Instruction::Srli(inst) => {
                let shamt = inst.imm & 0b111111; // Shift amount
                self.setreg(inst.rd, self.getreg(inst.rs1) >> shamt);
            }
            Instruction::Sub(inst) => {
                self.setreg(
                    inst.rd,
                    self.getreg(inst.rs1).wrapping_sub(self.getreg(inst.rs2)),
                );
            }
            Instruction::Sw(inst) => {
                let address = self.getreg(inst.rs1).wrapping_add(inst.imm as i64 as u64) as usize;
                let value = self.getreg(inst.rs2);
                self.memory[address] = value as u8;
                self.memory[address + 1] = (value >> 8) as u8;
                self.memory[address + 2] = (value >> 16) as u8;
                self.memory[address + 3] = (value >> 24) as u8;
            }
            Instruction::Xor(inst) => {
                self.setreg(inst.rd, self.getreg(inst.rs1) ^ self.getreg(inst.rs2))
            }
            Instruction::Xori(inst) => {
                self.setreg(inst.rd, (inst.imm as i64 as u64) ^ self.getreg(inst.rs1))
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

    pub fn print_state(&self) {
        for i in (0..32).step_by(4) {
            print!("{:>3} = 0x{:08x} ", format!("x{}", i), self.regs[i]);
            print!("{:>3} = 0x{:08x} ", format!("x{}", i + 1), self.regs[i + 1]);
            print!("{:>3} = 0x{:08x} ", format!("x{}", i + 2), self.regs[i + 2]);
            print!(
                "{:>3} = 0x{:08x}\n",
                format!("x{}", i + 3),
                self.regs[i + 3]
            );
        }
        println!(" pc = 0x{:08x}", self.pc);
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
            0x17, 0x05, 0x00, 0x00, // auipc a0,0x0
            0x03, 0x25, 0x05, 0x00, // lw    a0,0(a0)
        ];

        let mut emu = Emulator::new(code);
        emu.run();

        assert_eq!(emu.getreg(10), 0x00000517);
    }

    #[test]
    fn test_andi_ori_xori() {
        let code = vec![
            0x13, 0x05, 0x50, 0x00, // li   a0,5
            0x93, 0x75, 0x45, 0x00, // andi a1,a0,4
            0x13, 0x66, 0x25, 0x00, // ori  a2,a0,2
            0x93, 0x46, 0xa5, 0x00, // xori a3,a0,10
        ];

        let mut emu = Emulator::new(code);
        emu.run();

        assert_eq!(emu.getreg(10), 0x00000005);
        assert_eq!(emu.getreg(11), 0x00000004);
        assert_eq!(emu.getreg(12), 0x00000007);
        assert_eq!(emu.getreg(13), 0x0000000f);
    }
}
