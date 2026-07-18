use syn::{parse_str, Ident, Type};

use crate::FunctionSignature;
use std::collections::HashMap;

use quote::{format_ident, quote};

use crate::signature::SignatureInfo;

pub fn generate(
    bindgen_result: &SignatureInfo,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut content = String::new();

    let (known_stubs, unknown_stubs) = generate_function_stubs(&bindgen_result.signatures)?;

    content.push_str("\n\n");
    content.push_str(include_str!("./rustlib.tpl.rs"));
    content.push_str("\n\n");
    content.push_str(&known_stubs);
    content.push_str("\n\n");
    content.push_str(&unknown_stubs);
    content.push_str("\n\n");
    content.push_str(&bindgen_result.bindings);
    Ok(content)
}

fn generate_function_stubs(
    signature_map: &HashMap<String, FunctionSignature>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut known_stubs = Vec::new();

    for (func, sig) in signature_map {
        let func_name: Ident = parse_str(func)?;
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
            .filter_map(|(name, ty)| {
                let arg = format_ident!("{}", name);
                let typ: Type = parse_str(&ty).ok()?;
                Some(quote! { #arg: #typ })
            })
            .collect();

        let return_type: Type = parse_str(&sig.return_type)?;

        let stub_code = quote! {
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
                tracing::trace!("@entry");
                let #func_name: extern "C" fn(#(#args),*) -> #return_type = unsafe { std::mem::transmute(get_sym(#func)) };
                let ret = match unsafe { #func_name::HANDLE } {
                    Some(handler) => handler(#func_name, #(#params),*),
                    None => #func_name(#(#params),*),
                };
                tracing::trace!("@exit");
                ret
            }
        };

        let tree = &syn::parse2(stub_code.clone())?;
        let code = prettyplease::unparse(tree);

        known_stubs.push(code);
    }

    Ok((known_stubs.join("\n"), "".to_string()))
}
