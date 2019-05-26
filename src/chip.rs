use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Chip {
    pub name: String,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub version: Option<String>,

    pub peripherals: HashMap<String, Peripheral>,
    pub interrupts: HashMap<String, Interrupt>,
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

#[derive(Debug, Clone)]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub index: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum AccessMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug, Clone)]
pub enum ValueRestriction {
    Unsafe,
    Any,
    Range(usize, usize),
    Enumerated(HashMap<String, EnumeratedValue>),
}

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub description: Option<String>,
    pub address: usize,
    pub size: usize,
    pub access: AccessMode,
    pub restriction: ValueRestriction,

    pub fields: HashMap<String, Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub range: (usize, usize),
    pub access: AccessMode,
    pub restriction: ValueRestriction,
}

impl Field {
    pub fn width(&self) -> usize {
        self.range.1 - self.range.0 + 1
    }
}

#[derive(Debug, Clone)]
pub struct EnumeratedValue {
    pub name: String,
    pub description: Option<String>,
    pub value: usize,
}
