pub mod header;

pub use header::BindgenProvider;

pub trait SignatureProvider {
    fn get_signatures(
        &self,
        so_path: &str,
    ) -> Result<crate::BindgenResult, Box<dyn std::error::Error>>;
}
