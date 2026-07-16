use quote::{format_ident, quote};
use syn::{parse_str, Ident, Type};

use crate::FunctionSignature;
use std::{collections::HashMap, str::FromStr};

pub fn generate_function_stubs(
    functions: &[String],
    signatures: &HashMap<String, FunctionSignature>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut known_stubs = String::new();
    let mut unknown_stubs = String::new();

    for func in functions {
        let func_name: Ident = parse_str(func)?;
        let pre_call =
            proc_macro2::TokenStream::from_str(&format!("eprintln!(\"[CALL] {}\")", func))?;
        let post_call = proc_macro2::TokenStream::from_str("")?;

        let stub_code = if let Some(sig) = signatures.get(func) {
            let params: Vec<_> = sig
                .params
                .iter()
                .map(|(name, _)| format_ident!("{}", name))
                .collect();

            let args: Vec<_> = sig
                .params
                .iter()
                .filter_map(|typ| {
                    let arg = format_ident!("{}", typ.0);
                    let typ: Type = parse_str(&typ.1).ok()?;
                    Some(quote! { #arg: #typ })
                })
                .collect();

            let return_type: Type = parse_str(&sig.return_type)?;

            quote! {
                #[no_mangle]
                pub unsafe extern "C" fn #func_name(#(#args),*) -> #return_type {
                    let #func_name: extern "C" fn(#(#args),*) -> #return_type = std::mem::transmute(get_sym(#func));
                    #pre_call;
                    let ret = #func_name(#(#params),*);
                    #post_call;
                    ret
                }
            }
        } else {
            // Generic stub - unknown signature
            quote! {
                #[no_mangle]
                pub unsafe extern "C" fn #func_name() {
                    let #func_name: extern "C" fn() = std::mem::transmute(get_sym(#func));
                    #pre_call;
                    #func_name();
                    #post_call;
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
