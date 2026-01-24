use quote::{format_ident, quote};
use syn::{parse_str, Ident, Type};

use crate::FunctionSignature;
use std::collections::HashMap;

pub fn generate_function_stubs(
    functions: &[String],
    signatures: &HashMap<String, FunctionSignature>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut known_stubs = String::new();
    let mut unknown_stubs = String::new();

    for func in functions {
        let func_name: Ident = parse_str(&func)?;

        let stub_code = if let Some(sig) = signatures.get(func) {
            let params: Vec<_> = sig
                .params
                .iter()
                .enumerate()
                .map(|(i, _)| format_ident!("arg{}", i))
                .collect();

            let args: Vec<_> = sig
                .params
                .iter()
                .enumerate()
                .filter_map(|(i, typ)| {
                    let arg = format_ident!("arg{}", i);
                    let typ: Type = parse_str(typ).ok()?;
                    Some(quote! { #arg: #typ })
                })
                .collect();

            let return_type: Type = parse_str(&sig.return_type)?;

            quote! {
                #[no_mangle]
                pub unsafe extern "C" fn #func_name(#(#args),*) -> #return_type {
                    let #func_name: extern "C" fn(#(#args),*) -> #return_type = std::mem::transmute(get_sym(#func));
                    eprintln!("[CALL] {}", #func);
                    #func_name(#(#params),*)
                }
            }
        } else {
            // Generic stub - unknown signature
            quote! {
                #[no_mangle]
                pub unsafe extern "C" fn #func_name() {
                    let #func_name: extern "C" fn() = std::mem::transmute(get_sym(#func));
                    eprintln!("[CALL] {}", #func);
                    #func_name()
                }
            }
        };

        if signatures.contains_key(func) {
            known_stubs.push_str(&stub_code.to_string());
        } else {
            unknown_stubs.push_str(&stub_code.to_string());
        }
    }

    Ok((known_stubs, unknown_stubs))
}
