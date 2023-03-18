use eframe::{
    egui::{CentralPanel, ComboBox, Layout, RichText, TextEdit, TopBottomPanel, Window},
    App, CreationContext,
};
use ureq::{json, post};

use crate::structures::{AccInfo, AuthResp, Sex::*, SignupPage, WARN};

impl SignupPage {
    pub fn _new(_cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for SignupPage {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let uri = "http://localhost:8090/";
        TopBottomPanel::top("head").show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(eframe::egui::Direction::LeftToRight),
                |ui| {
                    ui.label(
                        RichText::new("Signup")
                            .text_style(eframe::egui::TextStyle::Monospace)
                            .strong()
                            .size(23.),
                    );
                },
            )
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Fullname: ");
            ui.add_space(5.);
            ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                ui.text_edit_singleline(&mut self.fullname);
                if self.fullname.is_empty() {
                    self.fnamerrvisi = true;
                    ui.colored_label(
                        WARN,
                        RichText::new("FullName can't be empty!")
                            .text_style(eframe::egui::TextStyle::Monospace),
                    );
                } else {
                    self.fnamerrvisi = false;
                }
            });
            ui.add_space(3.);
            ui.label("Username: ");
            ui.add_space(5.);
            ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                ui.text_edit_singleline(&mut self.username);

                if ui.button("🔎").clicked() {
                    match post(&format!("{uri}ch_name"))
                        .set("Content-Type", "application/json")
                        .send_json(json! ({
                            "username": self.username.clone()
                        })) {
                        Ok(_) => {
                            self.nameava = Some(true);
                            self.namechd = true;
                        }
                        Err(ureq::Error::Status(400, _)) => {
                            self.namerror = "Username is Not Available!".into();
                            self.nameava = Some(false);
                            self.namechd = false;
                            self.nameerrvisi = true;
                        }
                        Err(_) => {
                            self.namerror = "Something is wrong Try again!".into();
                            self.nameava = Some(false);
                            self.namechd = false;
                            self.nameerrvisi = true;
                        }
                    }
                }

                if self.nameerrvisi {
                    ui.colored_label(WARN, RichText::new(&self.namerror));
                }
            });
            ui.add_space(3.);
            ui.label("Password: ");
            ui.add_space(5.);
            ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                ui.add(TextEdit::singleline(&mut self.password).password(!self.pass));
                if self.passerrvivi {
                    ui.colored_label(WARN, RichText::new(&self.passerrr));
                }
                let pass = ui.button("><");

                if pass.clicked() {
                    self.pass = !self.pass;
                }
            });
            ui.add_space(3.);
            ComboBox::from_label("Choose Gender Sex!")
                .selected_text(format!("{:?}", self.sex))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sex, Male, "Male");
                    ui.selectable_value(&mut self.sex, Female, "Female");
                    ui.selectable_value(&mut self.sex, Custom, "Custom (Don't wanna say)");
                });
            ui.add_space(7.);
            match (self.username.is_empty(), self.password.is_empty()) {
                (false, false) => {
                    if self.namechd && self.nameava.unwrap() {
                        self.nameerrvisi = false;
                    }
                    if self.password.len() < 8 {
                        self.passerrvivi = true;
                        self.passerrr = "Password Shouldn't be less than 8!".into();
                    } else {
                        self.passerrvivi = false;
                        if !self.passerrvivi
                            && !self.nameerrvisi
                            && self.namechd
                            && !self.fnamerrvisi
                        {
                            if ui.button("Signup").clicked() {
                                match post(&format!("{uri}auth/signup"))
                                    .set("Content-Type", "application/json")
                                    .send_json(json!({
                                            "signup": {
                                            "fullname": self.fullname.clone(),
                                            "username": self.username.clone(),
                                            "password": self.password.clone(),
                                            "sex": self.sex
                                        }
                                    })) {
                                    Ok(resp) => {
                                        let jresp: AuthResp = resp.into_json().unwrap();
                                        let acc_cfg = AccInfo {
                                            authd: true,
                                            token: jresp.value,
                                        };
                                        confy::store("tracebook", Some("AccInfo"), acc_cfg)
                                            .unwrap();
                                    }
                                    Err(ureq::Error::Status(400, _)) => {
                                        self.namechd = false;
                                        self.nameerrvisi = false;
                                        self.nameava = Some(false);
                                        self.namerror = "Username is Not Available!".into();
                                    }
                                    Err(_) => {
                                        Window::new("Error").open(&mut self.errwin).show(
                                            ctx,
                                            |ui| {
                                                ui.label(
                                                    "Something went wrong please Try again later!",
                                                );
                                            },
                                        );
                                    }
                                }
                            }
                        }
                    }
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
                }
                _ => {
                    self.nameerrvisi = true;
                    self.namerror = "Username can't be empty!".to_string();
                    self.passerrvivi = true;
                    self.passerrr = "Password can't be empty!".to_string();
                }
            }
        });
    }
}
