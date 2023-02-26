#![windows_subsystem = "windows"]

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

mod config;
mod gui;

const TICKET_IDENTIFIER: &str = "{TICKET}";

#[tokio::main]
async fn main() {
    let config_location = &config::location().await.expect("Unable to get config dir");
    let config = config::load_configuration(config_location).await;
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("There must be a clipboard");

    let Ok(result) = ctx.get_contents() else {
        println!("Unable to load clipboard");
        return;
    };
    let result = result.to_uppercase();
    let result = result.trim();

    for org in &config.organisations {
        if result.starts_with(org) {
            let out = config.jira_prefix.replace(TICKET_IDENTIFIER, result);

            println!("Open page: {out}");
            open::that(out).expect("Unable to open browser");
            return;
        }
    }

    // When the clipboard does not contain any a ticket open a gui
    gui::run(config, config_location.to_string())
}
