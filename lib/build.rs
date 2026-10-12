fn main() {
    // Tauri's manifest is only embedded in the app binary. Unit tests link the
    // same Windows UI stack, so they need Common Controls v6 on their own.
    let attributes = tauri_build::Attributes::new()
        .windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());

    if let Err(error) = tauri_build::try_build(attributes) {
        panic!("failed to run tauri build script: {error}");
    }

    let manifest = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("windows-test.manifest");
    let manifest = short_path(&manifest);

    println!("cargo:rerun-if-changed={manifest}");
    println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg=/MANIFESTINPUT:{manifest}");
}

#[cfg(windows)]
fn short_path(path: &std::path::Path) -> String {
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetShortPathNameW(long_path: *const u16, short_path: *mut u16, buffer: u32) -> u32;
    }

    let long: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let mut buffer = vec![0u16; 512];
    let length = unsafe { GetShortPathNameW(long.as_ptr(), buffer.as_mut_ptr(), buffer.len() as u32) };

    if length == 0 || length as usize >= buffer.len() {
        return path.display().to_string();
    }

    String::from_utf16_lossy(&buffer[..length as usize])
}

#[cfg(not(windows))]
fn short_path(path: &std::path::Path) -> String {
    path.display().to_string()
}
