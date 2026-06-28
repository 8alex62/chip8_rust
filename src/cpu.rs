use std::fmt::{Display};
use std::assert_matches;

use crate::errors::CpuError;
use crate::instructions::Instruction;

const MEMORY_SIZE: usize = 4096; // 4KB of memory
const REGISTERS_SIZE: usize = 16; // 16 general-purpose registers
const STACK_SIZE: usize = 16; // 16 levels of stack
const PROGRAM_START: u16 = 0x200; // Program counter starts at 0x200 (512)

#[derive(Debug)]
pub struct CPU {
    memory: [u8; MEMORY_SIZE],
    v: [u8; REGISTERS_SIZE],
    pc: u16,
    stack: [u16; STACK_SIZE],
    jump: [u16; STACK_SIZE],
    jump_nb: usize,
}

/// The CPU struct represents the state of a Chip-8 emulator,
///  including its memory, registers, program counter, and stack.
impl CPU {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            v: [0; REGISTERS_SIZE], 
            pc: PROGRAM_START,
            stack: [0; STACK_SIZE],
            jump: [0; STACK_SIZE], // Initialize jump stack with 16 levels
            jump_nb: 0, // Initialize jump stack pointer
        }
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU State:\nMemory: {:?}\nRegisters: {:?}\nProgram Counter: {}\nStack: {:?}\nJump Stack: {:?}\nJump Stack Pointer: {}",
            self.memory, self.v, self.pc, self.stack, self.jump, self.jump_nb
        )
    }
}

impl CPU {
    // Writes the value at the specified address if in bounds
    fn write_memory(&mut self, address: usize, value: u8) -> Result<(), CpuError> {
        if address >= self.memory.len() || address < PROGRAM_START as usize {
            return Err(CpuError::MemoryAddressOutOfBounds((address, PROGRAM_START as usize, MEMORY_SIZE as usize)))
        }

        self.memory[address] = value;
        Ok(())
    }

    // Gets the next opcode from the memory
    fn get_next_opcode(self) -> u16 {
        let pc = self.pc;
        let high_byte = self.memory[pc as usize] as u16;
        let low_byte = self.memory[(pc + 1) as usize] as u16;
        (high_byte << 8) | low_byte
    }

    // Executes the instruction given in parameters
    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), CpuError> {
        match instruction {
            Instruction::JpAddr { address } => self.jump(address),
            Instruction::CallAddr { address } => self.call(address),
            Instruction::SeVxByte { vx, nn } => self.skip_e(vx, nn),
            Instruction::SneVxByte { vx, nn } => self.skip_ne(vx, nn),
            _ => return Err(CpuError::UnknownInstruction)
        }

        Ok(())
    }

    // Jumps to the address given in parameters -> 0x1NNN
    fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    // Calls the function at the specified address
    fn call(&mut self, address: u16) {
        // TODO : implementing the logic
    }

    // Skips next instruction if vx == nn
    fn skip_e(&mut self, vx: usize, nn: u8) {
        if self.v[vx] == nn {
            self.pc += 2;
        }
    }

    fn skip_ne(&mut self, vx: usize, nn: u8) {
        if self.v[vx] != nn {
            self.pc += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_new() {
        let cpu = CPU::new();
        assert_eq!(cpu.memory, [0; MEMORY_SIZE]);
        assert_eq!(cpu.v, [0; REGISTERS_SIZE]);
        assert_eq!(cpu.pc, PROGRAM_START);
        assert_eq!(cpu.stack, [0; STACK_SIZE]);
        assert_eq!(cpu.jump, [0; STACK_SIZE]);
        assert_eq!(cpu.jump_nb, 0); 
    }

    #[test]
    fn test_get_next_opcode() {
        // Initialize the memory with the opcode
        let mut cpu = CPU::new();
        cpu.memory[0x200] = 0x80;
        cpu.memory[0x201] = 0x10;

        let opcode = cpu.get_next_opcode();
        assert_eq!(opcode, 0x8010);
    }

    #[test]
    fn test_write_memory_ok() {
        let mut cpu = CPU::new();

        let _ = cpu.write_memory(0x200, 0x80);

        assert_eq!(cpu.memory[0x200], 0x80)
    }

    #[test]
    fn test_write_memory_nok() {
        let mut cpu = CPU::new();

        let high_limit = cpu.write_memory(MEMORY_SIZE, 0x80);
        let low_limit = cpu.write_memory(PROGRAM_START as usize - 1, 0x80);

        assert!(high_limit.is_err());
        assert_matches!(high_limit.unwrap_err(), CpuError::MemoryAddressOutOfBounds((_, _, _)));
        
        assert!(low_limit.is_err());
        assert_matches!(low_limit.unwrap_err(), CpuError::MemoryAddressOutOfBounds((_, _, _)));
    }

    #[test]
    fn test_execute_instruction_jump() {
        let mut cpu = CPU::new();
        let jump_instruction = Instruction::JpAddr { address: PROGRAM_START + 1 };

        let _ = cpu.execute_instruction(jump_instruction);

        assert_eq!(cpu.pc, 0x201);
    }

    #[test]
    fn test_execute_instruction_skip_e() {
        let mut cpu = CPU::new();
        let skip_e_instruction_e = Instruction::SeVxByte { vx: 1, nn: 0 };
        let skip_e_instruction_ne = Instruction::SeVxByte { vx: 1, nn: 1 };

        let _ = cpu.execute_instruction(skip_e_instruction_e);
        assert_eq!(cpu.pc, 0x202);

        let _ = cpu.execute_instruction(skip_e_instruction_ne);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_execute_instruction_skip_ne() {
        let mut cpu = CPU::new();
        let skip_ne_instruction_e = Instruction::SneVxByte { vx: 1, nn: 0 };
        let skip_ne_instruction_ne = Instruction::SneVxByte { vx: 1, nn: 1 };

        let _ = cpu.execute_instruction(skip_ne_instruction_e);
        assert_eq!(cpu.pc, 0x200);

        let _ = cpu.execute_instruction(skip_ne_instruction_ne);
        assert_eq!(cpu.pc, 0x202);
    }
}
