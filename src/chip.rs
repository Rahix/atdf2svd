use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Chip {
    pub name: String,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub version: Option<String>,

    pub peripherals: HashMap<String, Peripheral>,
}

#[derive(Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub description: Option<String>,

    pub registers: HashMap<String, Register>,
}

impl Peripheral {
    pub fn base_address(&self) -> Option<usize> {
        self.registers.values().map(|r| r.address).min()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AccessMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub description: Option<String>,
    pub address: usize,
    pub access: AccessMode,
}
