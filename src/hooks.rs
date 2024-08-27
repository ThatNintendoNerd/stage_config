pub(crate) mod gravity;
mod ground;
mod settings;
mod stage;

/// Installs all the function hooks and memory patches.
pub fn install() {
    skyline::install_hooks!(
        stage::stage_base_pre_setup,
        stage::is_flat_stage,
        settings::set_stage_random_settings,
    );

    stage::patch_create_stage_jump_table();
}
