mod config;
mod hooks;
mod offsets;

#[skyline::main(name = "stage_config")]
fn main() {
    hooks::install();
}
