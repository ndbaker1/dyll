use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn copy_templates_to_output(
    output_dir: &Path,
    replacements: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    fn copy_and_replace(
        src: &Path,
        dst: &Path,
        replacements: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if src.is_dir() {
            fs::create_dir_all(dst)?;
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());
                copy_and_replace(&src_path, &dst_path, replacements)?;
            }
        } else {
            let mut content = fs::read_to_string(src)?;
            for (key, value) in replacements {
                // NOTE: this somewhat hard to read string is just converting
                // the occurences of {{KEY}} to VALUE.
                content = content.replace(&format!("{{{{{key}}}}}"), value);
            }
            fs::write(dst, content)?;
        }
        Ok(())
    }

    // CARGO_MANIFEST_DIR allows us to read the contents of the project.
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/project-template");
    copy_and_replace(&template_path, output_dir, replacements)
}

pub fn generate_project(
    output_dir: &Path,
    lib_path: &Path,
    known_stubs: &str,
    unknown_stubs: &str,
    bindings: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let lib_path_str = if lib_path.is_absolute() {
        lib_path.to_str().unwrap().to_string()
    } else {
        // two levels up because the resulting file will be under src/lib.rs
        format!("../../{}", lib_path.display())
    };

    let mut replacements = HashMap::new();
    replacements.insert("FUNCTION_STUBS".to_string(), known_stubs.to_string());
    replacements.insert(
        "UNKNOWN_FUNCTION_STUBS".to_string(),
        unknown_stubs.to_string(),
    );
    replacements.insert("SHARED_LIBRARY_PATH".to_string(), lib_path_str.to_string());
    replacements.insert("BINDGEN_BINDINGS".to_string(), bindings.to_string());

    copy_templates_to_output(output_dir, &replacements)?;

    Ok(())
}
