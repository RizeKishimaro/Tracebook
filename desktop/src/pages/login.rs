use eframe::{
    egui::{CentralPanel, Layout, RichText, TextEdit, TopBottomPanel, Window},
    epaint::Color32,
    App, CreationContext,
};
use ureq::json;

use crate::structures::LoginPage;

const WARN: Color32 = Color32::from_rgb(255, 121, 0);

impl LoginPage {
    fn _new(_cc: CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for LoginPage {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("bar").show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(eframe::egui::Direction::LeftToRight),
                |ui| {
                    ui.label(
                        RichText::new("Login")
                            .text_style(eframe::egui::TextStyle::Monospace)
                            .strong()
                            .size(23.),
                    )
                },
            )
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Username: ");
            ui.add_space(5.);
            ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                ui.text_edit_singleline(&mut self.username);
                if self.nameerrvisi {
                    ui.colored_label(
                        WARN,
                        RichText::new(&self.namerror)
                            .text_style(eframe::egui::TextStyle::Monospace),
                    );
                }
            });

            ui.add_space(3.);

            ui.label("Password: ");
            ui.add_space(5.);
            ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                ui.add(TextEdit::singleline(&mut self.password).password(!self.passshow));
                if ui.button("><").clicked() {
                    self.passshow = !self.passshow;
                }

                if self.passerrvisi {
                    ui.colored_label(
                        WARN,
                        RichText::new(&self.passerrr)
                            .text_style(eframe::egui::TextStyle::Monospace),
                    );
                }
            });
            ui.add_space(7.);
            if self.username.is_empty() {
                self.nameerrvisi = true;
                self.namerror = "Username can't be empty!".into();
            } else {
                self.nameerrvisi = false;
            }

            if self.password.len() < 8 {
                self.passerrvisi = true;
                self.passerrr = "Password can't be less than 8 letters!".into();
            } else {
                self.passerrvisi = false;
            }

            if !self.passerrvisi && !self.nameerrvisi {
                if ui.button("Login").clicked() {
                    match ureq::post("http://localhost:8090/auth/login")
                        .set("Content-Type", "application/json")
                        .send_json(json!({
                            "login": {
                            "username": self.username.clone(),
                            "password": self.password.clone()
                        }
                        })) {
                        Ok(_) => todo!(),
                        Err(ureq::Error::Status(400, _)) => {
                            self.nameerrvisi = true;
                            self.namerror = "User Not Found!".into();
                        }
                        Err(ureq::Error::Status(401, _)) => {
                            self.passerrvisi = true;
                            self.passerrr = "Password Incorrect!".into();
                        }
                        Err(_) => {
                            Window::new("Error").show(ctx, |ui| {
                                ui.label("Something went wrong please Try again later!");
                            });
                        }
                    }
                }
            }
        });
    }
}
