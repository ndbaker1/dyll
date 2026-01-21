// stub_generator/src/main.rs
use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use slibbery::{DwarfProvider, FunctionSignature, HeaderProvider, SignatureProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: {} <input.so> <output_dir> [header.h]", args[0]);
        std::process::exit(1);
    }

    let so_path = &args[1];
    let output_dir = PathBuf::from(&args[2]);
    let header_path = if args.len() == 4 {
        Some(&args[3])
    } else {
        None
    };

    let buffer = fs::read(so_path)?;
    let elf = Elf::parse(&buffer)?;

    // Extract function symbols
    let mut functions = Vec::new();

    for sym in &elf.dynsyms {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if sym.is_function() && !name.is_empty() && sym.st_value != 0 {
                functions.push(name.to_string());
            }
        }
    }

    println!("Found {} functions", functions.len());

    // Get function signatures from appropriate provider
    let provider: Box<dyn SignatureProvider> = if let Some(header) = header_path {
        println!("Using header file: {}", header);
        Box::new(HeaderProvider::new(header.to_string()))
    } else {
        println!("Using DWARF debug info");
        Box::new(DwarfProvider)
    };

    let signatures = provider.get_signatures(&elf, &buffer).unwrap_or_default();
    println!("Parsed {} function signatures", signatures.len());

    // Generate lib.rs
    let lib_code = generate_lib_code(&functions, &signatures)?;
    fs::create_dir_all(output_dir.join("src"))?;
    fs::write(output_dir.join("src/lib.rs"), lib_code)?;

    // Copy original .so to project
    fs::copy(so_path, output_dir.join("original.so"))?;

    // Generate Cargo.toml
    let cargo_toml = r#"[package]
name = "mock_lib"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"

[lib]
crate-type = ["cdylib"]
"#;
    fs::write(output_dir.join("Cargo.toml"), cargo_toml)?;

    println!("Generated stub library in {}", output_dir.display());
    println!(
        "Build with: cd {} && cargo build --release",
        output_dir.display()
    );

    Ok(())
}

fn generate_lib_code(
    functions: &[String],
    signatures: &HashMap<String, FunctionSignature>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut code = format!(
        r#"use std::sync::{{Once}};

use std::os::raw::c_void;

static mut LIB_HANDLE: Option<*mut c_void> = None;
static INIT: Once = Once::new();

// Embed original library
const ORIGINAL_SO: &[u8] = include_bytes!("../original.so");

#[cfg(target_os = "linux")]
fn load_embedded_lib() -> *mut c_void {{
    unsafe {{
        // Create anonymous file in memory
        let fd = libc::syscall(libc::SYS_memfd_create, c"embedded_lib".as_ptr(), 0);
        if fd < 0 {{
            panic!("memfd_create failed");
        }}
        
        // Write library to memfd
        let written = libc::write(fd as i32, ORIGINAL_SO.as_ptr() as *const c_void, ORIGINAL_SO.len());
        if written != ORIGINAL_SO.len() as isize {{
            panic!("Failed to write to memfd");
        }}
        
        // Create path to memfd
        let path = format!("/proc/self/fd/{{}}\0", fd);
        
        // Load library from memfd
        let handle = libc::dlopen(path.as_ptr() as *const i8, libc::RTLD_LAZY);
        if handle.is_null() {{
            panic!("Failed to dlopen embedded library");
        }}
        
        handle
    }}
}}

fn get_lib() -> *mut c_void {{
    unsafe {{
        INIT.call_once(|| {{
            LIB_HANDLE = Some(load_embedded_lib());
        }});
        LIB_HANDLE.unwrap()
    }}
}}

unsafe fn get_symbol(name: &[u8]) -> *const c_void {{
    let handle = get_lib();
    let sym = libc::dlsym(handle, name.as_ptr() as *const i8);
    if sym.is_null() {{
        panic!("Symbol not found: {{}}", std::str::from_utf8(name).unwrap());
    }}
    sym
}}

"#
    );

    // Generate stub for each function
    for func in functions {
        let stub_code = if let Some(sig) = signatures.get(func) {
            // Generate type-aware stub
            let params_str = sig.params.join(", ");
            let params_call = if sig.params.is_empty() {
                "".to_string()
            } else {
                (0..sig.params.len())
                    .map(|i| format!("arg{}", i))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let args_def = if sig.params.is_empty() {
                "()".to_string()
            } else {
                format!(
                    "({})",
                    (0..sig.params.len())
                        .map(|i| format!("arg{}: {}", i, sig.params[i]))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            format!(
                r#"
#[no_mangle]
pub extern "C" fn {func}{args_def} -> {return_type} {{
    // add your mock logic here.
    unsafe {{
        let orig: extern "C" fn{args_def} -> {return_type} = std::mem::transmute(get_symbol(b"{func}\0"));
        orig({params_call})
    }}
}}
"#,
                func = func,
                args_def = args_def,
                return_type = sig.return_type,
                params_call = params_call
            )
        } else {
            // Fallback to generic stub
            format!(
                r#"
#[no_mangle]
pub extern "C" fn {func}() {{
    // add your mock logic here.
    unsafe {{
        let orig: extern "C" fn() = std::mem::transmute(get_symbol(b"{func}\0"));
        orig()
    }}
}}
"#,
                func = func
            )
        };
        code.push_str(&stub_code);
    }

    Ok(code)
}
