mod config;
mod hooks;
mod mem;
mod offsets;
mod service;

#[skyline::main(name = "stage_config")]
fn main() {
    hooks::install();
    mem::write();
}
