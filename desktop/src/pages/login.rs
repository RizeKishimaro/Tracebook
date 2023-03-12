use eframe::{
    egui::{CentralPanel, Layout, RichText, TextEdit, TopBottomPanel},
    epaint::Color32,
    App, CreationContext,
};
use serde::{Deserialize, Serialize};

type CLR = (u8, u8, u8);

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LoginPage {
    pub username: String,
    pub password: String,
    pub pass: bool,
    pub namerror: String,
    pub namerrclr: CLR,
    pub nameerrvisi: bool,
    pub passerrr: String,
    pub passerrclr: CLR,
    pub passerrvivi: bool,
}

impl LoginPage {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for LoginPage {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("head").show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(eframe::egui::Direction::LeftToRight),
                |ui| {
                    ui.label(
                        RichText::new("Login")
                            .text_style(eframe::egui::TextStyle::Monospace)
                            .strong()
                            .size(23.),
                    );
                },
            )
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Username: ");
            ui.add_space(5.);
            ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                ui.text_edit_singleline(&mut self.username);
                if self.nameerrvisi {
                    ui.group(|ui| {
                        let vec_cl = &self.namerrclr;
                        ui.colored_label(
                            Color32::from_rgb(vec_cl.0, vec_cl.1, vec_cl.2),
                            RichText::new(&self.namerror),
                        );
                    });
                }
            });
            ui.add_space(3.);
            ui.label("Password: ");
            ui.add_space(5.);
            ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                ui.add(TextEdit::singleline(&mut self.password).password(!self.pass));
                if self.passerrvivi {
                    ui.group(|ui| {
                        let vec_cl = &self.passerrclr;
                        ui.colored_label(
                            Color32::from_rgb(vec_cl.0, vec_cl.1, vec_cl.2),
                            RichText::new(&self.passerrr),
                        );
                    });
                }
                let pass = ui.button("><");

                if pass.clicked() {
                    self.pass = !self.pass;
                }
            });
            ui.add_space(7.);
            match (self.username.is_empty(), self.password.is_empty()) {
                (false, false) => {
                    self.passerrvivi = false;
                    self.nameerrvisi = false;
                    ui.button("Login");
                }
                (true, false) => {
                    self.nameerrvisi = true;
                    self.passerrvivi = false;
                    self.namerror = "Username can't be empty!".to_string();
                }
                (false, true) => {
                    self.passerrvivi = true;
                    self.nameerrvisi = false;
                    self.passerrr = "Password can't be empty!".to_string();
                    self.passerrclr = (255, 121, 0);
                }
                _ => {
                    self.nameerrvisi = true;
                    self.namerror = "Username can't be empty!".to_string();
                    self.namerrclr = (255, 121, 0);
                    self.passerrvivi = true;
                    self.passerrr = "Password can't be empty!".to_string();
                    self.passerrclr = (255, 121, 0);
                }
            }
        });
    }
}
