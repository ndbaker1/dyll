use gimli::read::EndianSlice;
use gimli::{Dwarf, LittleEndian};
use goblin::elf::Elf;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

use crate::FunctionSignature;

pub trait SignatureProvider {
    fn get_signatures(
        &self,
        elf: &Elf,
        buffer: &[u8],
    ) -> Result<HashMap<String, FunctionSignature>, Box<dyn std::error::Error>>;
}

pub struct DwarfProvider;

impl SignatureProvider for DwarfProvider {
    fn get_signatures(
        &self,
        elf: &Elf,
        buffer: &[u8],
    ) -> Result<HashMap<String, FunctionSignature>, Box<dyn std::error::Error>> {
        let mut signatures = HashMap::new();

        // Find DWARF sections
        let mut debug_info_data = &[][..];
        let mut debug_abbrev_data = &[][..];
        let mut debug_str_data = &[][..];

        for section in &elf.section_headers {
            if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                match name {
                    ".debug_info" => {
                        debug_info_data =
                            &buffer[section.sh_offset as usize..][..section.sh_size as usize];
                    }
                    ".debug_abbrev" => {
                        debug_abbrev_data =
                            &buffer[section.sh_offset as usize..][..section.sh_size as usize];
                    }
                    ".debug_str" => {
                        debug_str_data =
                            &buffer[section.sh_offset as usize..][..section.sh_size as usize];
                    }
                    _ => {}
                }
            }
        }

        if debug_info_data.is_empty() {
            // No DWARF info available
            return Ok(signatures);
        }

        // Load DWARF
        let dwarf: Dwarf<EndianSlice<'_, LittleEndian>> = Dwarf::load(
            |section| -> Result<EndianSlice<'_, LittleEndian>, gimli::Error> {
                use gimli::SectionId::*;
                match section {
                    DebugInfo => Ok(EndianSlice::new(debug_info_data, LittleEndian)),
                    DebugAbbrev => Ok(EndianSlice::new(debug_abbrev_data, LittleEndian)),
                    DebugStr => Ok(EndianSlice::new(debug_str_data, LittleEndian)),
                    _ => Ok(EndianSlice::new(&[], LittleEndian)),
                }
            },
        )?;

        // Parse compilation units
        let mut units = dwarf.units();
        while let Some(header) = units.next()? {
            let unit = dwarf.unit(header)?;

            // Parse DIEs
            let mut entries = unit.entries();
            while let Some((_, entry)) = entries.next_dfs()? {
                if entry.tag() == gimli::DW_TAG_subprogram {
                    if let Some(func_sig) = parse_function_signature(&dwarf, &unit, entry)? {
                        signatures.insert(func_sig.name.clone(), func_sig);
                    }
                }
            }
        }

        Ok(signatures)
    }
}

fn parse_function_signature(
    dwarf: &Dwarf<EndianSlice<'_, LittleEndian>>,
    unit: &gimli::Unit<EndianSlice<'_, LittleEndian>>,
    entry: &gimli::DebuggingInformationEntry<EndianSlice<'_, LittleEndian>>,
) -> Result<Option<FunctionSignature>, Box<dyn std::error::Error>> {
    // Get function name
    let name = if let Some(name_attr) = entry.attr(gimli::DW_AT_name)? {
        match name_attr.value() {
            gimli::AttributeValue::String(name_str) => name_str.to_string()?.to_string(),
            gimli::AttributeValue::DebugStrRef(offset) => {
                let name_str = dwarf.debug_str.get_str(offset)?;
                name_str.to_string()?.to_string()
            }
            _ => return Ok(None),
        }
    } else {
        return Ok(None);
    };

    // Get return type
    let return_type = if let Some(type_attr) = entry.attr(gimli::DW_AT_type)? {
        dwarf_type_to_rust(dwarf, unit, type_attr.value())?
    } else {
        "()".to_string() // void
    };

    // Get parameters
    let mut params = Vec::new();
    let mut children = unit.entries_at_offset(entry.offset())?;
    while let Some((_, child)) = children.next_dfs()? {
        if child.tag() == gimli::DW_TAG_formal_parameter {
            if let Some(type_attr) = child.attr(gimli::DW_AT_type)? {
                let param_type = dwarf_type_to_rust(dwarf, unit, type_attr.value())?;
                params.push(param_type);
            }
        } else if child.tag() == gimli::DW_TAG_subprogram {
            // Next function
            break;
        }
    }

    Ok(Some(FunctionSignature {
        name,
        params,
        return_type,
    }))
}

fn dwarf_type_to_rust(
    dwarf: &Dwarf<EndianSlice<'_, LittleEndian>>,
    unit: &gimli::Unit<EndianSlice<'_, LittleEndian>>,
    type_value: gimli::AttributeValue<EndianSlice<'_, LittleEndian>>,
) -> Result<String, Box<dyn std::error::Error>> {
    match type_value {
        gimli::AttributeValue::UnitRef(offset) => {
            let type_die = unit.entry(offset)?;
            match type_die.tag() {
                gimli::DW_TAG_base_type => {
                    // Get encoding and byte size
                    let encoding = type_die
                        .attr(gimli::DW_AT_encoding)?
                        .map(|attr: gimli::Attribute<EndianSlice<'_, LittleEndian>>| {
                            match attr.value() {
                                gimli::AttributeValue::Encoding(enc) => Some(enc),
                                _ => None,
                            }
                        })
                        .flatten();

                    let byte_size = type_die
                        .attr(gimli::DW_AT_byte_size)?
                        .map(|attr: gimli::Attribute<EndianSlice<'_, LittleEndian>>| {
                            match attr.value() {
                                gimli::AttributeValue::Data1(size) => Some(size as u64),
                                gimli::AttributeValue::Data2(size) => Some(size as u64),
                                gimli::AttributeValue::Data4(size) => Some(size as u64),
                                gimli::AttributeValue::Data8(size) => Some(size as u64),
                                _ => None,
                            }
                        })
                        .flatten()
                        .unwrap_or(0);

                    match encoding {
                        Some(gimli::DW_ATE_signed) => match byte_size {
                            1 => Ok("i8".to_string()),
                            2 => Ok("i16".to_string()),
                            4 => Ok("i32".to_string()),
                            8 => Ok("i64".to_string()),
                            _ => Ok("i32".to_string()), // default
                        },
                        Some(gimli::DW_ATE_unsigned) => match byte_size {
                            1 => Ok("u8".to_string()),
                            2 => Ok("u16".to_string()),
                            4 => Ok("u32".to_string()),
                            8 => Ok("u64".to_string()),
                            _ => Ok("u32".to_string()), // default
                        },
                        Some(gimli::DW_ATE_float) => match byte_size {
                            4 => Ok("f32".to_string()),
                            8 => Ok("f64".to_string()),
                            _ => Ok("f64".to_string()), // default
                        },
                        _ => Ok("c_void".to_string()), // unknown
                    }
                }
                gimli::DW_TAG_pointer_type => Ok("*mut c_void".to_string()),
                _ => Ok("c_void".to_string()), // unsupported type
            }
        }
        _ => Ok("c_void".to_string()),
    }
}

pub fn c_type_to_rust(c_type: &str) -> String {
    match c_type.trim() {
        "int" => "i32".to_string(),
        "unsigned int" | "unsigned" => "u32".to_string(),
        "long" => "i64".to_string(),
        "unsigned long" => "u64".to_string(),
        "short" => "i16".to_string(),
        "unsigned short" => "u16".to_string(),
        "char" => "i8".to_string(),
        "unsigned char" => "u8".to_string(),
        "float" => "f32".to_string(),
        "double" => "f64".to_string(),
        "void" => "()".to_string(),
        t if t.ends_with('*') => "*mut c_void".to_string(),
        _ => "c_void".to_string(), // fallback
    }
}

pub struct HeaderProvider {
    header_path: String,
}

impl HeaderProvider {
    pub fn new(header_path: String) -> Self {
        HeaderProvider { header_path }
    }
}

impl SignatureProvider for HeaderProvider {
    fn get_signatures(
        &self,
        _elf: &Elf,
        _buffer: &[u8],
    ) -> Result<HashMap<String, FunctionSignature>, Box<dyn std::error::Error>> {
        self.get_signatures_from_header_only()
    }
}

impl HeaderProvider {
    pub fn get_signatures_from_header_only(
        &self,
    ) -> Result<HashMap<String, FunctionSignature>, Box<dyn std::error::Error>> {
        let mut signatures = HashMap::new();

        // Read header file
        let content = fs::read_to_string(&self.header_path)?;

        // Parse function declarations using regex
        // This is a simple implementation - in practice, you'd want a proper C parser

        // Match function declarations like: int add(int a, int b);
        let func_regex = Regex::new(
            r"(?m)^\s*([a-zA-Z_][a-zA-Z0-9_*\s]+)\s+([a-zA-Z_][a-zA-Z0-9_]+)\s*\(([^)]*)\)\s*;",
        )?;

        for cap in func_regex.captures_iter(&content) {
            let return_type = cap[1].trim().to_string();
            let func_name = cap[2].trim().to_string();
            let params_str = cap[3].trim();

            // Parse parameters
            let mut params = Vec::new();
            if !params_str.is_empty() && params_str != "void" {
                // Simple parameter parsing - split by comma and extract types
                for param in params_str.split(',') {
                    let param = param.trim();
                    if let Some(type_end) = param.rfind(|c: char| !c.is_alphanumeric() && c != '_')
                    {
                        let param_type = param[..=type_end].trim().to_string();
                        params.push(c_type_to_rust(&param_type));
                    }
                }
            }

            signatures.insert(
                func_name.clone(),
                FunctionSignature {
                    name: func_name,
                    params,
                    return_type: c_type_to_rust(&return_type),
                },
            );
        }

        Ok(signatures)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_header_provider_simple_function() {
        let header_content = r#"
int add(int a, int b);
void print_hello(void);
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, header_content).unwrap();

        let provider = HeaderProvider::new(temp_file.path().to_str().unwrap().to_string());
        // For header provider, elf and buffer are not used, so we can pass dummy values
        // Since Elf doesn't have a Default impl, we'll create a minimal test without it
        let signatures = provider.get_signatures_from_header_only().unwrap();

        assert_eq!(signatures.len(), 2);

        let add_sig = signatures.get("add").unwrap();
        assert_eq!(add_sig.name, "add");
        assert_eq!(add_sig.params, vec!["i32", "i32"]);
        assert_eq!(add_sig.return_type, "i32");

        let hello_sig = signatures.get("print_hello").unwrap();
        assert_eq!(hello_sig.name, "print_hello");
        assert_eq!(hello_sig.params.len(), 0);
        assert_eq!(hello_sig.return_type, "()");
    }

    #[test]
    fn test_header_provider_complex_types() {
        let header_content = r#"
unsigned int get_count(void);
float calculate(double value, char* name);
long process_data(short id, unsigned long flags);
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, header_content).unwrap();

        let provider = HeaderProvider::new(temp_file.path().to_str().unwrap().to_string());
        // For header provider, elf and buffer are not used, so we can pass dummy values
        // Since Elf doesn't have a Default impl, we'll create a minimal test without it
        let signatures = provider.get_signatures_from_header_only().unwrap();

        assert_eq!(signatures.len(), 3);

        let get_count_sig = signatures.get("get_count").unwrap();
        assert_eq!(get_count_sig.return_type, "u32");
        assert_eq!(get_count_sig.params.len(), 0);

        let calc_sig = signatures.get("calculate").unwrap();
        assert_eq!(calc_sig.return_type, "f32");
        assert_eq!(calc_sig.params, vec!["f64", "*mut c_void"]);

        let process_sig = signatures.get("process_data").unwrap();
        assert_eq!(process_sig.return_type, "i64");
        assert_eq!(process_sig.params, vec!["i16", "u64"]);
    }

    #[test]
    fn test_header_provider_with_macros() {
        let header_content = r#"
#define DECLDIR __attribute__((visibility("default")))

nvmlReturn_t DECLDIR nvmlInit_v2(void);
nvmlReturn_t DECLDIR nvmlShutdown(void);
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, header_content).unwrap();

        let provider = HeaderProvider::new(temp_file.path().to_str().unwrap().to_string());
        // For header provider, elf and buffer are not used, so we can pass dummy values
        // Since Elf doesn't have a Default impl, we'll create a minimal test without it
        let signatures = provider.get_signatures_from_header_only().unwrap();

        assert_eq!(signatures.len(), 2);

        let init_sig = signatures.get("nvmlInit_v2").unwrap();
        assert_eq!(init_sig.name, "nvmlInit_v2");
        assert_eq!(init_sig.params.len(), 0);
        assert_eq!(init_sig.return_type, "c_void"); // nvmlReturn_t not recognized

        let shutdown_sig = signatures.get("nvmlShutdown").unwrap();
        assert_eq!(shutdown_sig.name, "nvmlShutdown");
        assert_eq!(shutdown_sig.params.len(), 0);
        assert_eq!(shutdown_sig.return_type, "c_void");
    }

    #[test]
    fn test_c_type_to_rust_mapping() {
        assert_eq!(c_type_to_rust("int"), "i32");
        assert_eq!(c_type_to_rust("unsigned int"), "u32");
        assert_eq!(c_type_to_rust("long"), "i64");
        assert_eq!(c_type_to_rust("unsigned long"), "u64");
        assert_eq!(c_type_to_rust("short"), "i16");
        assert_eq!(c_type_to_rust("unsigned short"), "u16");
        assert_eq!(c_type_to_rust("char"), "i8");
        assert_eq!(c_type_to_rust("unsigned char"), "u8");
        assert_eq!(c_type_to_rust("float"), "f32");
        assert_eq!(c_type_to_rust("double"), "f64");
        assert_eq!(c_type_to_rust("void"), "()");
        assert_eq!(c_type_to_rust("char*"), "*mut c_void");
        assert_eq!(c_type_to_rust("some_unknown_type"), "c_void");
    }
}
