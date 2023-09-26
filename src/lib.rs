use once_cell::sync::Lazy;

mod config;
mod hooks;
mod offsets;

use config::CONFIG;
use offsets::OFFSETS;

#[skyline::main(name = "stage_config")]
fn main() {
    Lazy::force(&OFFSETS);
    skyline::install_hooks!(
        hooks::stage::stage_base_pre_setup,
        hooks::stage::is_flat_stage,
        hooks::settings::set_stage_random_settings,
    );
    Lazy::force(&CONFIG);
    hooks::stage::patch_create_stage_jump_table();
}
