use crate::FunctionSignature;

pub mod header;

pub trait SignatureProvider {
    fn get_signatures(&self) -> Result<SignatureInfo, Box<dyn std::error::Error>>;
}

#[derive(Debug, Default)]
pub struct SignatureInfo {
    pub signatures: std::collections::HashMap<String, FunctionSignature>,
    pub bindings: String,
}
