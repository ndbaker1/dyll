use std::fs;
use std::path::Path;

pub fn generate_project(
    output_dir: &Path,
    known_stubs: &str,
    unknown_stubs: &str,
    bindings: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/templates/lib.tpl.rs");
    let content = fs::read_to_string(template_path)?;

    fs::write(
        output_dir.join("dyll.rs"),
        format!("{content}\n{known_stubs}\n{unknown_stubs}\n{bindings}").trim(),
    )?;

    Ok(())
}
