use eframe::{
    egui::{CentralPanel, Layout, RichText, TextEdit, TopBottomPanel},
    App, CreationContext,
};
use ureq::json;

use crate::structures::{err_win, AccInfo, AuthResp, LoginPage, WARN};

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
                    ui.colored_label(WARN, RichText::new(&self.passerrr));
                }
            });
            ui.add_space(7.);

            if ui.button("Login").clicked() {
                match ureq::post("http://localhost:8090/auth/login")
                    .set("Content-Type", "application/json")
                    .send_json(json!({
                        "login": {
                        "username": self.username.clone(),
                        "password": self.password.clone()
                    }
                    })) {
                    Ok(resp) => {
                        self.passerrvisi = false;
                        self.nameerrvisi = false;
                        match resp.into_json::<AuthResp>() {
                            Ok(jresp) => {
                                let acc_cfg = AccInfo {
                                    authd: true,
                                    token: jresp.value,
                                };
                                let oki = confy::store("tracebook", Some("AccInfo"), acc_cfg);
                                if oki.is_err() {
                                    self.errwin = true;
                                    err_win(ctx, &mut self.errwin, "Something went wrong please Report to Developer and Try again!");
                                }
                            }
                            Err(_) => {
                                self.errwin = true;
                                err_win(ctx, &mut self.errwin, "Something went wrong please Report to Developer and Try again!");
                            }
                        }
                    }
                    Err(ureq::Error::Status(400, _)) => {
                        self.nameerrvisi = true;
                        self.namerror = "User Not Found!".into();
                    }
                    Err(ureq::Error::Status(401, _)) => {
                        self.passerrvisi = true;
                        self.passerrr = "Password Incorrect!".into();
                    }
                    Err(_) => {
                        self.errwin = true;
                        err_win(ctx, &mut self.errwin, "Something went wrong please Report to Developer and Try again!");
                    }
                }
            }
        });
    }
}
