use eframe::egui;

use crate::{config::Configuration, TICKET_IDENTIFIER};

struct App {
    text: String,
    selected: SelectableOrg,
    config: Configuration,
    config_location: String,
}

#[derive(Debug, Default, PartialEq)]
struct SelectableOrg {
    pub prefix: String,
    pub organisation: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Select an organisation")
                .selected_text(&self.selected.organisation)
                .show_ui(ui, |ui| {
                    for instance in &self.config.instances {
                        for organisation in &instance.organisations {
                            ui.selectable_value(
                                &mut self.selected,
                                SelectableOrg {
                                    prefix: instance.prefix.clone(),
                                    organisation: organisation.clone(),
                                },
                                organisation,
                            );
                        }
                    }
                });
            ui.add(egui::TextEdit::singleline(&mut self.text).hint_text("ticket id number"));
            // if i.key_pressed(egui::Key::ArrowUp) {
            //     if self.organisation.is_empty() {
            //         self.organisation = next_organisation(
            //             NextDirection::Forward,
            //             None,
            //             &self.config.organisations,
            //         )
            //     } else {
            //         self.organisation = next_organisation(
            //             NextDirection::Forward,
            //             Some(&self.organisation),
            //             &self.config.organisations,
            //         )
            //     }
            // }
            if (ui.button("open").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)))
                && self.text.trim().parse::<usize>().is_ok()
            {
                let ticket = format!("{}-{}", self.selected.organisation, self.text.trim());
                let out = self
                    .selected
                    .prefix
                    .clone()
                    .replace(TICKET_IDENTIFIER, &ticket);

                println!("Open page: {out}");
                open::that(out).expect("Unable to open browser");
                todo!("stop");
            }
            ui.label(format!(
                "The config file is located at: {}",
                self.config_location
            ));
        });
    }
}

pub fn run(config: Configuration, config_location: String) {
    let app = App {
        text: String::from(""),
        selected: SelectableOrg::default(),
        config,
        config_location,
    };

    eframe::run_native(
        "jirclip",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|_| Box::new(app)),
    )
    .expect("Unable to open gui");
}

// pub enum NextDirection {
//     Forward,
//     Backward,
// }

// pub fn next_organisation(
//     direction: NextDirection,
//     cur: Option<&str>,
//     organisations: &Vec<String>,
// ) -> Option<String> {
//     let mut current = "";
//     if let Some(c) = cur {
//         current = c;
//     }
//     let current_index = organisations
//         .iter()
//         .enumerate()
//         .filter(|(_, val)| *val == current)
//         .map(|(idx, _)| idx)
//         .next();
//     let index;
//     match current_index {
//         Some(i) => index = i,
//         None => {
//             println!("Unable to find current_index");
//             return None;
//         }
//     }

//     let last = organisations.len() - 1;

//     organisations
//         .get(next_index(direction, index, last))
//         .cloned()
// }

// fn next_index(direction: NextDirection, index: usize, last: usize) -> usize {
//     match direction {
//         NextDirection::Forward => {
//             if index > last {
//                 //dbg!("1");
//                 return 0;
//             }
//             return index + 1;
//         }
//         NextDirection::Backward => {
//             if index == 0 {
//                 return last;
//             }
//             return index - 1;
//         }
//     }
// }

#[cfg(test)]
mod tests {

    #[test]
    fn next_org() {
        // let orgs = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        //assert_eq!(
        //"b",
        //next_organisation(NextDirection::Forward, orgs.get(0).unwrap(), &orgs).unwrap()
        //);
        //assert_eq!(
        //"c",
        //next_organisation(NextDirection::Forward, orgs.get(1).unwrap(), &orgs).unwrap()
        //);
        //
        // new
        // assert_eq!(
        //     "a",
        //     next_organisation(NextDirection::Forward, orgs.get(2).unwrap(), &orgs).unwrap()
        // );

        // assert_eq!(
        //     "b",
        //     next_organisation(NextDirection::Backward, orgs.get(0).unwrap(), &orgs).unwrap()
        // );
        // assert_eq!(
        //     "a",
        //     next_organisation(NextDirection::Backward, orgs.get(1).unwrap(), &orgs).unwrap()
        // );
        // assert_eq!(
        //     "c",
        //     next_organisation(NextDirection::Backward, orgs.get(2).unwrap(), &orgs).unwrap()
        // );
    }
}
