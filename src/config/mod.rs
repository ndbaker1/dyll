use std::path::{Path, PathBuf};

/// Returns the filesystem path of the .so this code is loaded from by scanning
/// `/proc/self/maps` for the mapping that contains the address of a function
/// in this crate.
///
/// Mirrors the approach used in NVIDIA k8s-test-infra's mock NVML engine
/// (`pkg/gpu/mocknvml/engine/config.go::discoverConfigPath`). Using a known
/// in-crate function address (rather than searching for a fixed SONAME) lets
/// the generated library identify itself regardless of what it was named to
/// impersonate at link time.
///
/// Returns `None` on non-Linux platforms or if discovery fails.
#[cfg(target_os = "linux")]
pub fn discover_self_so() -> Option<PathBuf> {
    use std::io::BufRead;

    let self_addr = discover_self_so as *const () as usize;

    let file = std::fs::File::open("/proc/self/maps").ok()?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let Ok(line) = line else { continue };
        // Format: addr-addr perms offset dev inode pathname
        let mut iter = line.split_whitespace();
        let Some(range) = iter.next() else { continue };
        let Some((start_str, end_str)) = range.split_once('-') else {
            continue;
        };
        let Ok(start) = usize::from_str_radix(start_str, 16) else {
            continue;
        };
        let Ok(end) = usize::from_str_radix(end_str, 16) else {
            continue;
        };
        if self_addr < start || self_addr >= end {
            continue;
        }

        // Skip perms/offset/dev/inode (4 fields) to land on the pathname.
        for _ in 0..4 {
            if iter.next().is_none() {
                return None;
            }
        }
        let mut pathname: String = iter.collect::<Vec<_>>().join(" ");
        // After a library replacement (e.g. upgrade in place), the kernel
        // appends " (deleted)" to the path. Strip it so we keep the real path.
        if let Some(trimmed) = pathname.strip_suffix(" (deleted)") {
            pathname = trimmed.to_string();
        }
        if !pathname.starts_with('/') {
            continue;
        }
        return Some(PathBuf::from(pathname));
    }
    None
}

#[cfg(not(target_os = "linux"))]
pub fn discover_self_so() -> Option<PathBuf> {
    None
}

/// Resolves a config path relative to the directory that contains this .so,
/// using the same two-level walk as the Go reference:
///
/// ```text
///   .so:    <root>/usr/lib64/lib<name>.so.<version>
///   config: <root>/<relative>
/// ```
///
/// `relative` is joined onto `<root>` (the .so's grandparent of its parent
/// directory). Returns `None` if discovery fails or the resulting path does
/// not exist on disk.
pub fn discover_config_path(relative: impl AsRef<Path>) -> Option<PathBuf> {
    let so_path = discover_self_so()?;
    let lib_dir = so_path.parent()?; // .../usr/lib64
    let root = lib_dir.parent()?.parent()?; // .../<root>
    let config = root.join(relative);
    config.exists().then_some(config)
}

/// Reads a single-value option file. Tries the .so-relative
/// `<root>/dyll/options/<name>` location first, then falls back to
/// `/etc/dyll/options/<name>`. Trailing whitespace is trimmed.
pub fn read_option(name: &str) -> Option<String> {
    let rel = PathBuf::from("dyll/options").join(name);
    let path = discover_config_path(&rel).or_else(|| {
        let fallback = PathBuf::from("/etc/dyll/options").join(name);
        fallback.exists().then_some(fallback)
    })?;
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
}
