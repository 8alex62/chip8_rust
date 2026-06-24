#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instructions {
    JpAddr,
    CallAddr,
    SeVxByte,
    SneVxByte,
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