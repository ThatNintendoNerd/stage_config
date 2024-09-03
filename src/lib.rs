mod config;
mod hooks;
mod offsets;
mod service;

#[skyline::main(name = "stage_config")]
fn main() {
    hooks::install();
}
