#[allow(unused_imports)]
#[allow(nonstandard_style)]
#[allow(non_snake_case)]
use libc::*;
use std::ffi::*;
use std::sync::Once;

extern "C" fn noop_fn() -> c_int {
    return 0;
}

pub fn get_sym(name: &str) -> *const c_void {
    let name_cstring = CString::new(name).expect("symbol should be convertible to cstring");
    let Ok(handle) = load_embedded_lib() else {
        // when the library is empty, just return a function that does nothing.
        return noop_fn as *const c_void;
    };
    let sym = unsafe { libc::dlsym(handle, name_cstring.as_ptr() as *const c_char) };
    if sym.is_null() {
        panic!("Symbol not found: {}", name);
    }
    sym
}

// the original shared library embedded in the program's static memory.
const ORIGINAL_SO: &[u8] = include_bytes!(env!("SHARED_LIBRARY_PATH"));

static mut LIB_HANDLE: Option<*mut c_void> = None;
static INIT: Once = Once::new();

/// Loads the embedded shared library using memfd_create instead of a tempfile.
///
/// memfd_create creates an anonymous file in memory that can be passed to dlopen.
/// Benefits over using a temporary file:
/// - No disk I/O: The library stays in memory, avoiding slow disk writes/reads
/// - Security: No temporary files left on disk that could be exploited or inspected
/// - Atomicity: The file is created and used entirely in memory, no filesystem state
/// - Cleanup: Automatically cleaned up when the process exits, no manual deletion needed
/// - Performance: Faster loading since no filesystem operations are involved
///
/// The memfd is created with a name for debugging purposes, then written with the
/// embedded library bytes. A path like /proc/self/fd/<fd> is constructed for dlopen.
#[cfg(target_os = "linux")]
fn load_embedded_lib() -> Result<*mut c_void, &'static str> {
    INIT.call_once(|| unsafe {
        // Create anonymous file in memory
        let fd = libc::syscall(libc::SYS_memfd_create, c"embedded_lib".as_ptr(), 0);
        if fd < 0 {
            panic!("memfd_create failed");
        }

        // Write library to memfd
        let written = libc::write(
            fd as i32,
            ORIGINAL_SO.as_ptr() as *const c_void,
            ORIGINAL_SO.len(),
        );
        if written != ORIGINAL_SO.len() as isize {
            panic!("Failed to write to memfd");
        }

        // Create path to memfd
        let path = format!("/proc/self/fd/{}\0", fd);

        // Load library from memfd
        let handle = libc::dlopen(path.as_ptr() as *const c_char, libc::RTLD_LAZY);
        if !handle.is_null() {
            LIB_HANDLE = Some(handle)
        } else {
            tracing::debug!("failed to dlopen library")
        }
    });
    unsafe { LIB_HANDLE.ok_or("missing handle") }
}

// catch-all for non-supported platforms.
#[cfg(not(target_os = "linux"))]
fn load_embedded_lib() -> Result<*mut c_void, &'static str> {
    unsafe { LIB_HANDLE.ok_or("missing handle") }
}
