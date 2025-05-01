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

    pub address: usize,
    pub register_group: RegisterGroup,
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
pub struct RegisterGroup {
    pub name: String,
    pub description: Option<String>,
    /// Offset relative to the peripheral base address
    pub offset: usize,
    /// Indicates if this register group is a union.
    ///
    /// Currently limits nested register group functionality.
    /// If removed, full nested register group support would be enabled (#4).
    pub is_union: bool,

    /// The register group references to other register groups. This is only used for filling up
    /// the subgroups.
    pub references: Vec<RegisterGroupReference>,
    pub subgroups: Vec<RegisterGroup>,
    pub registers: BTreeMap<String, Register>,
}

impl RegisterGroup {
    /// Recursively collects all registers from this register group and its subgroups.
    ///
    /// This function traverses the register group hierarchy, including all subgroups,
    /// and returns a collection of all registers found.
    pub fn get_all_registers(&self) -> Vec<&Register> {
        let mut registers = Vec::new();

        // Add registers from the current group
        registers.extend(self.registers.values());

        // Recursively add registers from all subgroups
        for subgroup in &self.subgroups {
            registers.extend(subgroup.get_all_registers());
        }

        // Return the collected registers
        registers
    }
}

#[derive(Debug, Clone)]
pub struct RegisterGroupReference {
    pub name: String,
    pub name_in_module: Option<String>,
    pub offset: Option<usize>,
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
    /// The absolute memory address where this register is located in the memory map
    pub address: usize,
    /// The relative offset of this register within its parent register group
    pub offset: usize,
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
