#[derive(Debug)]
pub struct Cpu {
    registers: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    program_counter: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self { registers: ([0; 16]), memory: ([0; 4096]), stack: ([0; 16]), program_counter: (0) }
    }
}

#[cfg(test)]
mod tests {

}