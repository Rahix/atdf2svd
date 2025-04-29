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
    pub name_in_module: String,
    pub description: Option<String>,

    pub registers: BTreeMap<String, Register>,
    pub register_group_headers: BTreeMap<String, RegisterGroupHeader>,
}

impl Peripheral {
    pub fn base_address(&self) -> Option<usize> {
        if self.is_union() {
            return self
                .get_union_register_group_headers()
                .iter()
                .flat_map(|(h, _)| h.registers.values())
                .map(|r| r.address)
                .min();
        }
        self.registers.values().map(|r| r.address).min()
    }

    pub fn module_register_group_header(&self) -> Option<&RegisterGroupHeader> {
        self.register_group_headers.get(&self.name_in_module)
    }

    pub fn is_union(&self) -> bool {
        let module_register_group_header = self.module_register_group_header();
        match module_register_group_header {
            Some(header) => header.is_union(),
            None => false,
        }
    }

    pub fn get_union_register_group_headers(
        &self,
    ) -> Vec<(&RegisterGroupHeader, &RegisterGroupItem)> {
        match self.module_register_group_header() {
            Some(module_header) if module_header.is_union() => module_header
                .register_group_items
                .values()
                .filter_map(|group_item| {
                    group_item.name_in_module.as_ref().and_then(|header_name| {
                        self.register_group_headers
                            .iter()
                            .find(|(_, header)| &header.name == header_name)
                            .map(|(_, header)| (header, group_item))
                    })
                })
                .collect(),
            _ => vec![],
        }
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
pub struct RegisterGroupHeader {
    pub name: String,
    pub class: Option<String>,
    pub description: Option<String>,
    pub size: Option<usize>,

    pub register_group_items: BTreeMap<String, RegisterGroupItem>,
    pub registers: BTreeMap<String, Register>,
}

impl RegisterGroupHeader {
    pub fn is_union(&self) -> bool {
        self.class.as_deref() == Some("union")
    }
}

#[derive(Debug, Clone)]
pub struct RegisterGroupItem {
    pub name: String,
    pub name_in_module: Option<String>,
    pub description: Option<String>,
    pub size: Option<usize>,
    pub offset: Option<usize>,
    pub count: Option<usize>,
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
