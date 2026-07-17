use quote::ToTokens;
use std::collections::HashMap;
use syn::{parse_str, FnArg::Typed, ForeignItem, Item, PatType, Type};

use super::SignatureProvider;
use crate::{providers::BindgenResult, FunctionSignature};

use crate::cli::{Cli, HeaderArgs};
use crate::elf;
use crate::generator;
use crate::templates;

pub fn run(cli: &Cli, args: &HeaderArgs) -> Result<(), Box<dyn std::error::Error>> {
    let functions: Vec<String> = elf::extract_function_symbols(&cli.lib_path)?
        .into_iter()
        .filter(|name| {
            if cli.skipped_symbols.contains(name) {
                tracing::info!("skipping symbol: {}", name);
                return false;
            }
            true
        })
        .collect();

    tracing::info!(
        "found {} function symbols in {}",
        functions.len(),
        cli.lib_path.display(),
    );

    let provider = BindgenProvider::new(args.header_file.to_str().unwrap().to_string());
    let bindgen_result = provider
        .get_signatures(cli.lib_path.to_str().unwrap())
        .unwrap_or_default();

    tracing::info!(
        "found {} function signatures in {}",
        bindgen_result.signatures.len(),
        args.header_file.display(),
    );

    let (known_stubs, unknown_stubs) =
        generator::generate_function_stubs(&functions, &bindgen_result.signatures)?;

    let lib_code = templates::generate_lib(&known_stubs, &unknown_stubs, &bindgen_result.bindings)?;

    std::fs::write(cli.output_dir.join("dyll.rs"), lib_code.trim())?;

    Ok(())
}

pub struct BindgenProvider {
    header_path: String,
}

impl BindgenProvider {
    pub fn new(header_path: String) -> Self {
        BindgenProvider { header_path }
    }
}

impl SignatureProvider for BindgenProvider {
    fn get_signatures(&self, _so_path: &str) -> Result<BindgenResult, Box<dyn std::error::Error>> {
        self.get_signatures_from_bindgen()
    }
}

impl BindgenProvider {
    pub fn get_signatures_from_bindgen(&self) -> Result<BindgenResult, Box<dyn std::error::Error>> {
        let mut signatures = HashMap::new();

        // Use bindgen to generate bindings from the header
        let bindings = bindgen::Builder::default()
            .header(&self.header_path)
            .generate_comments(false)
            .layout_tests(false)
            .default_enum_style(bindgen::EnumVariation::Consts)
            .prepend_enum_name(false)
            .array_pointers_in_arguments(true)
            .sort_semantically(true)
            .use_core()
            .generate()
            .map_err(|e| format!("Bindgen failed: {}", e))?;

        let generated_code = bindings.to_string();

        // Parse the generated code with syn
        let syntax_tree = parse_str::<syn::File>(&generated_code)?;

        let mut bindings_parts = Vec::new();

        for item in syntax_tree.items {
            match item {
                // TODO: i dont really love how this is done.
                Item::ForeignMod(ref foreign_mod) => {
                    // Check if it's extern "C"
                    let is_extern_c = foreign_mod
                        .abi
                        .name
                        .as_ref()
                        .map(|n| n.value() == "C")
                        .unwrap_or(false);
                    if is_extern_c {
                        // Extract function signatures from extern "C" blocks
                        for foreign_item in &foreign_mod.items {
                            if let ForeignItem::Fn(func) = foreign_item {
                                let name = func.sig.ident.to_string();
                                let return_type = match &func.sig.output {
                                    syn::ReturnType::Default => "()".to_string(),
                                    syn::ReturnType::Type(_, ty) => {
                                        ty.to_token_stream().to_string()
                                    }
                                };
                                let params = func
                                    .sig
                                    .inputs
                                    .iter()
                                    .filter_map(|arg| match arg {
                                        Typed(PatType { pat, ty, .. }) => Some((
                                            pat.to_token_stream().to_string(),
                                            ty.to_token_stream().to_string(),
                                        )),
                                        _ => None,
                                    })
                                    .collect();

                                signatures.insert(
                                    name.clone(),
                                    FunctionSignature {
                                        name,
                                        params,
                                        return_type,
                                    },
                                );
                            }
                        }
                    } else {
                        // Non-C extern blocks go into bindings
                        bindings_parts.push(format!("{}", quote::quote!(#item)));
                    }
                }
                _ => {
                    // Other items like type definitions, consts, etc.
                    bindings_parts.push(format!("{}", quote::quote!(#item)));
                }
            }
        }

        Ok(BindgenResult {
            signatures,
            bindings: bindings_parts.join("\n"),
        })
    }
}
