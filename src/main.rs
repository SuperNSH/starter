//! Runs the Rust application directly on the machine's native backend.

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(target_os = "linux")]
use nsh_platform_linux as platform;
#[cfg(target_os = "macos")]
use nsh_platform_macos as platform;
#[cfg(target_os = "windows")]
use nsh_platform_windows as platform;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn main() -> impl std::process::Termination {
    env_logger::init();
    // The desktop's deep-link seam: `nsh-starter nsh-starter://data/item/3`
    // opens that route once the shell is up.
    for arg in std::env::args().skip(1) {
        if arg.starts_with(nsh_starter::shell::deep_link::SCHEME) {
            nsh_starter::shell::deep_link::from_launch(arg);
        }
    }
    platform::run_app(
        platform::AppSource::Rust(platform::Root::of::<nsh_starter::App>()),
        platform::AppOptions {
            title: "nsh-starter".to_owned(),
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            app_id: "com.example.nshstarter".to_owned(),
            // Wide enough for the desktop shell (the sidebar), per the
            // shell's 700-point threshold on the desktops.
            size: (1280, 800),
            ..Default::default()
        },
    )
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!(
    "Run this binary on macOS, Windows or Linux; use the TSX entry with nsh run for iOS and Android."
);
