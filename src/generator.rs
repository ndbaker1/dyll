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
        let func_name: Ident = parse_str(func)?;
        let stub_code = if let Some(sig) = signatures.get(func) {
            let params: Vec<_> = sig
                .params
                .iter()
                .map(|(name, _)| format_ident!("{}", name))
                .collect();

            let types: Vec<Type> = sig
                .params
                .iter()
                .filter_map(|(_, ty)| parse_str(ty).ok())
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
                #[allow(unused)]
                pub mod #func_name {
                    use super::*;
                    pub static mut HANDLE: Option<&'static dyn Fn(extern "C" fn(#(#args),*) -> #return_type, #(#types),*) -> #return_type> = None;
                    pub fn register_handler(handle: &'static impl Fn(extern "C" fn(#(#args),*) -> #return_type, #(#types),*) -> #return_type) {
                        unsafe { HANDLE = Some(handle) }
                    }
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #func_name(#(#args),*) -> #return_type {
                    let span = tracing::span!(tracing::Level::TRACE, stringify!(#func_name));
                    let _enter = span.enter();
                    tracing::trace!("entry()");
                    let #func_name: extern "C" fn(#(#args),*) -> #return_type = unsafe { std::mem::transmute(get_sym(#func)) };
                    let ret = match unsafe { #func_name::HANDLE } {
                        Some(handler) => handler(#func_name, #(#params),*),
                        None => #func_name(#(#params),*),
                    };
                    tracing::trace!("exit()");
                    ret
                }
            }
        } else {
            // Generic stub - unknown signature
            quote! {
                #[allow(unused)]
                pub mod #func_name {
                    use super::*;
                    pub static mut HANDLE: Option<&'static dyn Fn(extern "C" fn())> = None;
                    pub fn register_handler(handler: &'static impl Fn(extern "C" fn())) {
                        unsafe { HANDLE = Some(handler) }
                    }
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn #func_name() {
                    let span = tracing::span!(tracing::Level::TRACE, stringify!(#func_name));
                    let _enter = span.enter();
                    tracing::trace!("entry()");
                    let #func_name: extern "C" fn() = unsafe { std::mem::transmute(get_sym(#func)) };
                    let ret = match unsafe { #func_name::HANDLE } {
                        Some(handler) => handler(#func_name),
                        None => #func_name(),
                    };
                    tracing::trace!("exit()");
                    ret
                }
            }
        };

        let tree = &syn::parse2(stub_code.clone())?;
        let code = &prettyplease::unparse(tree);

        if signatures.contains_key(func) {
            known_stubs.push_str(code);
        } else {
            unknown_stubs.push_str(code);
        }
    }

    Ok((known_stubs, unknown_stubs))
}
