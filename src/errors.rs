use std::{fmt::Display};

#[derive(Debug)]
pub enum CpuError {
    MemoryAddressOutOfBounds((usize, usize, usize))
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::MemoryAddressOutOfBounds((address, min, max)) => write!(f, "Memory address {address:?} out of bounds (between {min:?} and {max:?})"),
        }
    }
}