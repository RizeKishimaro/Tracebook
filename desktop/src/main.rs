use std::fs::remove_dir_all;

use directories::BaseDirs;
use eframe::{
    egui::{Hyperlink, RichText, TextStyle, TopBottomPanel},
    run_native, App, CreationContext, NativeOptions,
};
use structures::{AccInfo, MApp};

mod pages;
mod structures;

impl MApp {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn main() {
    let nt_opt = NativeOptions::default();
    run_native("MApp", nt_opt, Box::new(|cc| Box::new(MApp::new(cc)))).unwrap();
}

impl App for MApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let ri: AccInfo = match confy::load("tracebook", Some("AccInfo")) {
            Ok(aci) => aci,
            Err(_) => {
                let path = BaseDirs::new().unwrap();
                let path = path.config_dir().to_str().unwrap();
                let path = format!("{}/tracebook", &path[1..path.len() - 1]);
                remove_dir_all(path).unwrap();
                AccInfo {
                    authd: false,
                    token: String::new(),
                }
            }
        };
        if !ri.authd {
            TopBottomPanel::bottom("footer").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if !self.login {
                        self.pages.signup.update(ctx, frame);
                        if ui
                            .link(
                                RichText::new("Already have an account?")
                                    .text_style(TextStyle::Monospace),
                            )
                            .clicked()
                        {
                            self.login = true;
                        }
                    } else {
                        self.pages.login.update(ctx, frame);
                        if ui
                            .link(
                                RichText::new("Don't have an account?")
                                    .text_style(TextStyle::Monospace),
                            )
                            .clicked()
                        {
                            self.login = false;
                        }
                    }
                    ui.add(Hyperlink::from_label_and_url(
                        RichText::new("Made by Our Team!").text_style(TextStyle::Monospace),
                        "https://github.com/RizeKishimaro/Tracebook",
                    ));
                })
            });
        } else {
            self.pages.nf.update(ctx, frame);
        }
    }
}
