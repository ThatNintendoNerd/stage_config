mod config;
mod env;
mod hooks;
mod mem;
mod offsets;
mod service;

#[skyline::main(name = "stage_config")]
fn main() {
    if !env::is_app_version_compatible() {
        eprintln!(
            "[{}] The installed version of the target software is not compatible.",
            module_path!(),
        );
        return;
    }

    hooks::install();
    mem::write();
}
