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
