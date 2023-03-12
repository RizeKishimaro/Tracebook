use eframe::{App, CreationContext, NativeOptions, run_native};
use serde::{Deserialize, Serialize};
use structures::Pages;

mod pages;
mod structures;

#[derive(Debug, Default)]
struct MApp {
    pages: Pages,
}

impl MApp {
    fn new(cc: &CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn main() {
    let nt_opt = NativeOptions::default();
    run_native("MApp", nt_opt, Box::new(|cc| Box::new(MApp::new(cc))));
}

impl App for MApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.pages.login.update(ctx, frame);
    }
}
