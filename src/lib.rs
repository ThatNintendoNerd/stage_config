mod config;
mod env;
mod hooks;
mod mem;
mod offsets;
mod stage;

#[skyline::main(name = "stage_config")]
fn main() {
    if !env::is_app_version_compatible() {
        eprintln!(
            "[{}] The installed version of the target software is not compatible.",
            module_path!(),
        );
        return;
    }

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let payload = match info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => s.as_str(),
                None => "Box<Any>",
            },
        };
        let message = "stage_config has panicked! Please select Details and create an issue at https://github.com/ThatNintendoNerd/stage_config";
        let details = format!("thread panicked at {location}:\n{payload}");

        skyline::error::show_error(69, message, details.as_str());
    }));

    hooks::install();
    mem::write();
}
