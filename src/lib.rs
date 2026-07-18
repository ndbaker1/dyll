pub mod signature;
pub mod cli;
pub mod codegen;
pub mod elf;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
}
