use eframe::{egui, epaint::Vec2};

use crate::{config::Configuration, TICKET_IDENTIFIER};

struct App {
    text: String,
    organisation: String,
    config: Configuration,
    config_location: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select an organisation")
                .selected_text(self.organisation.as_str())
                .show_ui(ui, |ui| {
                    for organisation in &self.config.organisations {
                        ui.selectable_value(
                            &mut self.organisation,
                            organisation.to_string(),
                            organisation,
                        );
                    }
                });
            ui.add(egui::TextEdit::singleline(&mut self.text).hint_text("ticket id number"));
            if (ui.button("open").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)))
                && self.text.parse::<usize>().is_ok()
            {
                let ticket = format!("{}-{}", self.organisation, self.text.trim());
                let out = self.config.jira_prefix.replace(TICKET_IDENTIFIER, &ticket);

                println!("Open page: {out}");
                open::that(out).expect("Unable to open browser");

                frame.close();
            }
            ui.label(format!("The config file is located at: {}", self.config_location));
        });
    }
}

pub fn run(config: Configuration, config_location: String) {
    let mut default_organisation = String::from("");
    if let Some(first) = config.organisations.first() {
        default_organisation = first.to_string();
    };

    let app = App {
        text: String::from(""),
        organisation: default_organisation,
        config,
        config_location,
    };

    eframe::run_native(
        "jirclip",
        eframe::NativeOptions {
            initial_window_size: Option::from(Vec2::new(300_f32, 100_f32)),
            ..Default::default()
        },
        Box::new(|_| Box::new(app)),
    )
    .expect("Unable to open gui");
}
