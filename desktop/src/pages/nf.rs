use std::fs;

use eframe::{
    egui::{CentralPanel, Layout, RichText, ScrollArea, Separator, TopBottomPanel},
    App,
};
use ureq_multipart::MultipartBuilder;

use crate::structures::{AccInfo, NfPage, WARN};

impl App for NfPage {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let ri: AccInfo = match confy::load("Tracebook", Some("AccInfo")) {
            Ok(aci) => aci,
            Err(_) => {
                let path = BaseDirs::new().unwrap();
                let path = path.config_dir().to_str().unwrap();
                let path = format!("{}/Tracebook", &path[1..path.len() - 1]);
                remove_dir_all(path).unwrap();
                AccInfo {
                    authd: false,
                    token: String::new(),
                }
            }
        };
        if !ri.authd {
            TopBottomPanel::top("bar").show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("Tracebook"));
                ui.add_space(5.);
                ui.add(Separator::default().spacing(20.));
            });

            CentralPanel::default().show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                    ui.text_edit_multiline(&mut self.caption);
                    if ui.button("Image").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.image = Some(path.display().to_string())
                        }
                    }

                    if self.image.unwrap_or("".into()).is_empty() {
                        MultipartBuilder::new()
                            .add_text("token", &ri.token)
                            .unwrap()
                            .add_text("caption", &self.caption)
                    } else {
                        if infer::is_image(&fs::read(self.image.unwrap())) {
                            self.type_supt = Some(true);
                        } else {
                            self.type_supt = Some(false);
                        }
                    }

                    let post_btn = ui.button("Post");
                    if let Some(sup) = self.type_supt {
                        if !sup {
                            ui.colored_label(
                                WARN,
                                RichText::new("File Type Not Support!")
                                    .text_style(eframe::egui::TextStyle::Monospace),
                            );
                        }
                    }
                });
            });
        }
    }
}
