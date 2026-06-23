use std::fmt::Display;

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

// This function retrieves the opcode from the memory at the current program counter (pc).
// Because Chip-8 opcodes are 2 bytes long, 
// we read two consecutive bytes from memory and combine them into a single u16 value.
pub fn get_opcode(memory: &[u8], pc: u16) -> u16 {
    let high_byte = memory[pc as usize] as u16;
    let low_byte = memory[(pc + 1) as usize] as u16;
    (high_byte << 8) | low_byte
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
    fn test_get_opcode() {
        let memory = [0x80, 0x10, 0x00, 0x00]; // Example opcode: 8XY0
        let opcode = get_opcode(&memory, 0);
        assert_eq!(opcode, 0x8010);
    }
}
