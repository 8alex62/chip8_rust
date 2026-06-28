#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    JpAddr { address: u16},
    CallAddr { address: u16},
    SeVxByte { vx: usize, nn: u8},
    SneVxByte { vx: usize, nn: u8},
    SeVxVy,
    SneVxVy,
    LdVxByte,
    AddVxByte,
    Ret,
    LdVxVy,
    OrVxVy,
    AndVxVy,
    XorVxVy,
    AddVxVy,
    SubVxVy,
    SubnVxVy,
}