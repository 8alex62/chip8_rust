use crate::instructions::Instructions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

// The Rule struct represents a decoding rule for Chip-8 opcodes.
struct Rule {
    mask: u16,
    id: u16,
    instr: Instructions,
}

#[derive(Debug)]
pub struct OpcodeDecoder {
    table: [Rule; 16],
}

// The OpcodeDecoder struct is responsible for decoding Chip-8 opcodes into their corresponding instructions.
impl OpcodeDecoder {
    fn new() -> Self {
        Self {
            table: [
                Rule { mask: 0xF000, id: 0x1000, instr: Instructions::JpAddr },  /* 1NNN */
                Rule { mask: 0xF000, id: 0x2000, instr: Instructions::CallAddr },/* 2NNN */
                Rule { mask: 0xF000, id: 0x3000, instr: Instructions::SeVxByte },/* 3XNN */
                Rule { mask: 0xF000, id: 0x4000, instr: Instructions::SneVxByte },/* 4XNN */
                Rule { mask: 0xF00F, id: 0x5000, instr: Instructions::SeVxVy },  /* 5XY0 */
                Rule { mask: 0xF00F, id: 0x9000, instr: Instructions::SneVxVy }, /* 9XY0 */
                Rule { mask: 0xF000, id: 0x6000, instr: Instructions::LdVxByte },/* 6XNN */
                Rule { mask: 0xF000, id: 0x7000, instr: Instructions::AddVxByte },/* 7XNN */
                Rule { mask: 0xFFFF, id: 0x00EE, instr: Instructions::Ret },     /* 00EE */
                Rule { mask: 0xF00F, id: 0x8000, instr: Instructions::LdVxVy },  /* 8XY0 */
                Rule { mask: 0xF00F, id: 0x8001, instr: Instructions::OrVxVy },  /* 8XY1 */
                Rule { mask: 0xF00F, id: 0x8002, instr: Instructions::AndVxVy }, /* 8XY2 */
                Rule { mask: 0xF00F, id: 0x8003, instr: Instructions::XorVxVy }, /* 8XY3 */
                Rule { mask: 0xF00F, id: 0x8004, instr: Instructions::AddVxVy }, /* 8XY4 */
                Rule { mask: 0xF00F, id: 0x8005, instr: Instructions::SubVxVy }, /* 8XY5 */
                Rule { mask: 0xF00F, id: 0x8007, instr: Instructions::SubnVxVy },/* 8XY7 */
            ],
        }
    }

    // The decode method takes an opcode as input and returns the corresponding instruction if it matches any rule in the table.
    pub fn decode(&self, opcode: u16) -> Option<Instructions> {
        self.table
            .iter()
            .find(|rule| (opcode & rule.mask) == rule.id)
            .map(|rule| rule.instr)
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

        assert_eq!(decoder.decode(0x1ABC), Some(Instructions::JpAddr));
        assert_eq!(decoder.decode(0x2345), Some(Instructions::CallAddr));
        assert_eq!(decoder.decode(0x3AFF), Some(Instructions::SeVxByte));
        assert_eq!(decoder.decode(0x4A01), Some(Instructions::SneVxByte));
        assert_eq!(decoder.decode(0x5AB0), Some(Instructions::SeVxVy));
        assert_eq!(decoder.decode(0x9AB0), Some(Instructions::SneVxVy));
        assert_eq!(decoder.decode(0x6A10), Some(Instructions::LdVxByte));
        assert_eq!(decoder.decode(0x7A10), Some(Instructions::AddVxByte));
        assert_eq!(decoder.decode(0x00EE), Some(Instructions::Ret));
        assert_eq!(decoder.decode(0x8AB0), Some(Instructions::LdVxVy));
        assert_eq!(decoder.decode(0x8AB1), Some(Instructions::OrVxVy));
        assert_eq!(decoder.decode(0x8AB2), Some(Instructions::AndVxVy));
        assert_eq!(decoder.decode(0x8AB3), Some(Instructions::XorVxVy));
        assert_eq!(decoder.decode(0x8AB4), Some(Instructions::AddVxVy));
        assert_eq!(decoder.decode(0x8AB5), Some(Instructions::SubVxVy));
        assert_eq!(decoder.decode(0x8AB7), Some(Instructions::SubnVxVy));
    }

    #[test]
    fn test_decode_unknown_opcode() {
        let decoder = OpcodeDecoder::new();
        assert_eq!(decoder.decode(0x0000), None);
    }
}