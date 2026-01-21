pub mod header;

pub use header::HeaderProvider;

pub trait SignatureProvider {
    fn get_signatures(
        &self,
        so_path: &str,
    ) -> Result<
        std::collections::HashMap<String, crate::FunctionSignature>,
        Box<dyn std::error::Error>,
    >;
}
