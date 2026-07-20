use quote::ToTokens;
use std::collections::BTreeMap;
use syn::{parse_str, FnArg::Typed, ForeignItem, Item, PatType};

use super::SignatureProvider;
use crate::{signature::SignatureInfo, FunctionSignature};

pub struct BindgenProvider {
    header_path: String,
}

impl BindgenProvider {
    pub fn new(header_path: String) -> Self {
        BindgenProvider { header_path }
    }
}

impl SignatureProvider for BindgenProvider {
    fn get_signatures(&self) -> Result<SignatureInfo, Box<dyn std::error::Error>> {
        self.get_signatures_from_bindgen()
    }
}

impl BindgenProvider {
    pub fn get_signatures_from_bindgen(&self) -> Result<SignatureInfo, Box<dyn std::error::Error>> {
        let mut signatures = BTreeMap::new();

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
                        bindings_parts.push(item.to_token_stream().to_string());
                    }
                }
                _ => {
                    // Other items like type definitions, consts, etc.
                    bindings_parts.push(item.to_token_stream().to_string());
                }
            }
        }

        Ok(SignatureInfo {
            signatures,
            bindings: bindings_parts.join("\n"),
        })
    }
}
