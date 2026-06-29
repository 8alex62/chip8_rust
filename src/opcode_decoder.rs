use crate::instructions::Instruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// The Rule struct represents a decoding rule for Chip-8 opcodes.
struct Rule {
    mask: u16,
    id: u16,
}

#[derive(Debug)]
pub struct OpcodeDecoder {
    table: [Rule; 16],
}

// The OpcodeDecoder struct is responsible for decoding Chip-8 opcodes into their corresponding Instruction.
impl OpcodeDecoder {
    fn new() -> Self {
        Self {
            table: [
                Rule { mask: 0xF000, id: 0x1000 },  /* 1NNN */
                Rule { mask: 0xF000, id: 0x2000 },  /* 2NNN */
                Rule { mask: 0xF000, id: 0x3000 },  /* 3XNN */
                Rule { mask: 0xF000, id: 0x4000 },  /* 4XNN */
                Rule { mask: 0xF00F, id: 0x5000 },  /* 5XY0 */
                Rule { mask: 0xF00F, id: 0x9000 },  /* 9XY0 */
                Rule { mask: 0xF000, id: 0x6000 },  /* 6XNN */
                Rule { mask: 0xF000, id: 0x7000 },  /* 7XNN */
                Rule { mask: 0xFFFF, id: 0x00EE },  /* 00EE */
                Rule { mask: 0xF00F, id: 0x8000 },  /* 8XY0 */
                Rule { mask: 0xF00F, id: 0x8001 },  /* 8XY1 */
                Rule { mask: 0xF00F, id: 0x8002 },  /* 8XY2 */
                Rule { mask: 0xF00F, id: 0x8003 },  /* 8XY3 */
                Rule { mask: 0xF00F, id: 0x8004 },  /* 8XY4 */
                Rule { mask: 0xF00F, id: 0x8005 },  /* 8XY5 */
                Rule { mask: 0xF00F, id: 0x8007 },  /* 8XY7 */
            ],
        }
    }

    // The decode method takes an opcode as input and returns the corresponding instruction if it matches any rule in the table.
    pub fn decode(&self, opcode: u16) -> Option<Instruction> {
        let rule = self.table
            .iter()
            .position(|rule| (opcode & rule.mask) == rule.id);

        match rule {
            Some(i) => match i {
                0 => Some(Instruction::JpAddr { address: (opcode & 0x0FFF) }),
                1 => Some(Instruction::CallAddr { address: (opcode & 0x0FFF) }),
                2 => Some(Instruction::SeVxByte { vx: ((opcode & 0x0F00) >> 8) as usize, nn: (opcode & 0x00FF) as u8 }),
                3 => Some(Instruction::SneVxByte { vx: ((opcode & 0x0F00) >> 8) as usize, nn: (opcode & 0x00FF) as u8 }),
                4 => Some(Instruction::SeVxVy { vx: ((opcode & 0x0F00) >> 8) as usize, vy: ((opcode & 0x00F0) >> 4) as usize }),
                5 => Some(Instruction::SneVxVy { vx: ((opcode & 0x0F00) >> 8) as usize, vy: ((opcode & 0x00F0) >> 4) as usize }),
                6 => Some(Instruction::LdVxByte),
                7 => Some(Instruction::AddVxByte),
                8 => Some(Instruction::Ret),
                9 => Some(Instruction::LdVxVy),
                10 => Some(Instruction::OrVxVy),
                11 => Some(Instruction::AndVxVy),
                12 => Some(Instruction::XorVxVy),
                13 => Some(Instruction::AddVxVy),
                14 => Some(Instruction::SubVxVy),
                15 => Some(Instruction::SubnVxVy),
                _ => None
            },
            _ => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_new() {
        let decoder = OpcodeDecoder::new();
        assert_eq!(decoder.table.len(), 16);
        assert_eq!(decoder.table[0].mask, 0xF000);
        assert_eq!(decoder.table[0].id, 0x1000);
        assert_eq!(decoder.table[15].mask, 0xF00F);
        assert_eq!(decoder.table[15].id, 0x8007);
    }

    #[test]
    fn test_decode_known_opcodes() {
        let decoder = OpcodeDecoder::new();

        assert_eq!(decoder.decode(0x1ABC), Some(Instruction::JpAddr { address: 0x0ABC }));
        assert_eq!(decoder.decode(0x2345), Some(Instruction::CallAddr { address: 0x0345 }));
        assert_eq!(decoder.decode(0x3AFF), Some(Instruction::SeVxByte { vx: ((0x3AFF & 0x0F00) >> 8), nn: 0x00FF as u8 }));
        assert_eq!(decoder.decode(0x4A01), Some(Instruction::SneVxByte { vx: ((0x4A01 & 0x0F00) >> 8), nn: 0x0001 as u8 }));
        assert_eq!(decoder.decode(0x5AB0), Some(Instruction::SeVxVy { vx: ((0x5AB0 & 0x0F00) >> 8), vy: ((0x5AB0 & 0x00F0) >> 4) }));
        assert_eq!(decoder.decode(0x9AB0), Some(Instruction::SneVxVy { vx: ((0x9AB0 & 0x0F00) >> 8), vy: ((0x9AB0 & 0x00F0) >> 4) }));
        assert_eq!(decoder.decode(0x6A10), Some(Instruction::LdVxByte));
        assert_eq!(decoder.decode(0x7A10), Some(Instruction::AddVxByte));
        assert_eq!(decoder.decode(0x00EE), Some(Instruction::Ret));
        assert_eq!(decoder.decode(0x8AB0), Some(Instruction::LdVxVy));
        assert_eq!(decoder.decode(0x8AB1), Some(Instruction::OrVxVy));
        assert_eq!(decoder.decode(0x8AB2), Some(Instruction::AndVxVy));
        assert_eq!(decoder.decode(0x8AB3), Some(Instruction::XorVxVy));
        assert_eq!(decoder.decode(0x8AB4), Some(Instruction::AddVxVy));
        assert_eq!(decoder.decode(0x8AB5), Some(Instruction::SubVxVy));
        assert_eq!(decoder.decode(0x8AB7), Some(Instruction::SubnVxVy));
    }

    #[test]
    fn test_decode_unknown_opcode() {
        let decoder = OpcodeDecoder::new();
        assert_eq!(decoder.decode(0x0000), None);
    }
}