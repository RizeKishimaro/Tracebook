use eframe::{
    egui::{Hyperlink, RichText, TextStyle, TopBottomPanel},
    run_native, App, CreationContext, NativeOptions,
};
use structures::{AuthResp, MApp};

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
    }
}
