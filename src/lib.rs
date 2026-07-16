pub mod providers;

pub mod cli;
pub mod config;
pub mod elf;
pub mod generator;
pub mod templates;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
}
