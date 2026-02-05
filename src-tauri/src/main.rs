// Dark-GPT Desktop Application
// Entry point for the Tauri application

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    setup_panic_handler();
    dark_gpt_lib::run();
}

/// Install a panic hook that writes a crash log and shows a native
/// error dialog on Windows. Without this, `windows_subsystem = "windows"`
/// + `panic = "abort"` causes silent death with zero user feedback.
fn setup_panic_handler() {
    std::panic::set_hook(Box::new(|info| {
        let message = format!("Dark-GPT crashed unexpectedly:\n\n{}", info);

        // Write crash log next to executable
        if let Ok(exe) = std::env::current_exe() {
            let log_path = exe.with_file_name("dark-gpt-crash.log");
            let _ = std::fs::write(&log_path, &message);
        }

        // Show native error dialog on Windows
        #[cfg(target_os = "windows")]
        show_error_dialog("Dark-GPT — Fatal Error", &message);
    }));
}

#[cfg(target_os = "windows")]
fn show_error_dialog(title: &str, message: &str) {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;

    extern "system" {
        fn MessageBoxW(
            hwnd: *const core::ffi::c_void,
            text: *const u16,
            caption: *const u16,
            utype: u32,
        ) -> i32;
    }

    let title: Vec<u16> = OsStr::new(title).encode_wide().chain(once(0)).collect();
    let msg: Vec<u16> = OsStr::new(message).encode_wide().chain(once(0)).collect();

    unsafe {
        MessageBoxW(
            core::ptr::null(),
            msg.as_ptr(),
            title.as_ptr(),
            0x10, // MB_ICONERROR
        );
    }
}
