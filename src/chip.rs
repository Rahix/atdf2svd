use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Chip {
    pub name: String,
    pub architecture: String,
    pub family: String,
    pub series: Option<String>,

    pub description: Option<String>,
    pub vendor: Option<String>,
    pub version: Option<String>,

    pub peripherals: BTreeMap<String, Peripheral>,
    pub interrupts: BTreeMap<String, Interrupt>,
}

#[derive(Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub description: Option<String>,

    pub registers: BTreeMap<String, Register>,
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
    NoAccess,
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug, Clone)]
pub enum ValueRestriction {
    Unsafe,
    Any,
    Range(u64, u64),
    Enumerated(BTreeMap<String, EnumeratedValue>),
}

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
    pub description: Option<String>,
    pub mode: Option<String>,
    pub address: usize,
    pub size: usize,
    pub access: AccessMode,
    pub restriction: ValueRestriction,

    pub fields: BTreeMap<String, Field>,
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
