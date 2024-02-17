mod config;
mod hooks;
mod offsets;

#[skyline::main(name = "stage_config")]
fn main() {
    skyline::install_hooks!(
        hooks::stage::stage_base_pre_setup,
        hooks::stage::is_flat_stage,
        hooks::settings::set_stage_random_settings,
    );

    hooks::stage::patch_create_stage_jump_table();
}
