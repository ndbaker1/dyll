use crate::cli::{Cli, HeaderArgs};
use crate::elf;
use crate::generator;
use crate::providers::{BindgenProvider, SignatureProvider};
use crate::template;

pub fn run(cli: &Cli, args: &HeaderArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing library: {}", cli.lib_path.display());

    let functions = elf::extract_function_symbols(&cli.lib_path)?;
    println!("Found {} function symbols", functions.len());

    println!("Using header file: {}", args.header_file.display());
    let provider = BindgenProvider::new(args.header_file.to_str().unwrap().to_string());
    let bindgen_result = provider
        .get_signatures(cli.lib_path.to_str().unwrap())
        .unwrap_or_else(|_| crate::BindgenResult {
            signatures: std::collections::HashMap::new(),
            bindings: String::new(),
        });
    println!(
        "Parsed {} function signatures",
        bindgen_result.signatures.len()
    );

    let function_stubs =
        generator::generate_function_stubs(&functions, &bindgen_result.signatures)?;

    template::generate_project(
        &cli.output_dir,
        &cli.lib_path,
        &function_stubs,
        &bindgen_result.bindings,
    )?;

    println!("Generated stub library in {}", cli.output_dir.display());

    Ok(())
}
