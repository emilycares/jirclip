#![windows_subsystem = "windows"]

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub mod config;
mod gui;

const TICKET_IDENTIFIER: &str = "{TICKET}";
pub async fn run() {
    let (config_location, config) = get_config().await;
    let Some(clip) = get_clipboard() else {
        return;
    };

    if let Some(out) = get_url_for_ticket(&config, &clip) {
        println!("Open page: {out}");
        open::that(out).expect("Unable to open browser");
        return;
    }

    // When the clipboard does not contain any a ticket open a gui
    gui::run(config, config_location.to_string())
}

pub fn get_url_for_ticket(config: &config::Configuration, clip: &str) -> Option<String> {
    for instance in &config.instances {
        for org in &instance.organisations {
            if clip.starts_with(org.as_str()) {
                return Some(instance.prefix.replace(TICKET_IDENTIFIER, clip));
            }
        }
    }
    None
}

pub async fn get_config() -> (String, config::Configuration) {
    let config_location = &config::location().await.expect("Unable to get config dir");
    let config = config::load_configuration(config_location).await;
    (config_location.to_string(), config)
}

fn get_clipboard() -> Option<String> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("There must be a clipboard");

    let Ok(result) = ctx.get_contents() else {
        println!("Unable to load clipboard");
        return None;
    };
    let result = result.to_uppercase();
    let result = result.trim();
    Some(result.to_string())
}
