use eframe::egui::{self, ComboBox, Hyperlink, RichText, Separator, TopBottomPanel, Ui};
use opencv::{imgcodecs, videoio::VideoCapture};
use std::{fs, process::Command};

const PADDING: f32 = 5.0;

#[derive(PartialEq, Debug)]
enum Opti {
    Once,
    Loop,
}

fn main() {
    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Live Wallpaper Gui",
        native_options,
        Box::new(|cc| Box::new(Gui::new(cc))),
    );
}

struct Gui {
    picked_path: Option<String>,
    check: Opti,
}

impl Gui {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            picked_path: None,
            check: Opti::Once,
        }
    }
}

fn header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Live Wallpaper For Linux Written in Rust");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn footer(_ui: &mut Ui, ctx: &egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("Rize").monospace(),
                "https://facebook.com/RizeKishimaro",
            ));
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("Source Code").monospace(),
                "https://github.com/Walker-00/live-wallpaper",
            ));
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("Created By Walker").monospace(),
                "https://facebook.com/walker.fbi",
            ));
            ui.add_space(10.);
        })
    });
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut support = false;
        let Self {
            picked_path: _,
            check,
        } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            header(ui);
            ui.add_space(10.);
            ui.heading("Video File Format [mp4, mkv, etc] and gif format are supported!");
            ui.separator();
            ui.add_space(30.);
            ui.with_layout(
                egui::Layout::left_to_right(eframe::emath::Align::Min),
                |ui| {
                    ui.label("Choose File: ");
                    if ui.button("Choose Video...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                        }
                    }

                    if let Some(picked_path) = &self.picked_path {
                        ui.horizontal(|ui| {
                            ui.label("File Path:");
                            ui.monospace(picked_path);
                        });
                        let kind = infer::is_video(
                            &fs::read(
                                self.picked_path
                                    .to_owned()
                                    .as_ref()
                                    .expect("Something was wrong!"),
                            )
                            .unwrap(),
                        );
                        if kind {
                            ui.label("is supported");
                            support = true;
                        } else {
                            if infer::is(
                                &fs::read(
                                    self.picked_path
                                        .to_owned()
                                        .as_ref()
                                        .expect("Something was wrong!"),
                                )
                                .unwrap(),
                                "gif",
                            ) {
                                ui.label("is supported");
                                support = true;
                            } else {
                                ui.label("is not supported file type");
                                support = false;
                            }
                        }
                    }
                },
            );

            ComboBox::from_label("Choose Method")
                .selected_text(format!("{:?}", check))
                .show_ui(ui, |ui| {
                    ui.selectable_value(check, Opti::Once, "No Loop");
                    ui.selectable_value(check, Opti::Loop, "Loop");
                });

            if support {
                if ui.add(|ui: &mut Ui| ui.button("Process")).clicked() {
                    frame_and_idk(
                        self.picked_path
                            .as_ref()
                            .to_owned()
                            .expect("Something was wrong!"),
                        check,
                    );
                }
            }

            footer(ui, ctx);
        });
    }
}

fn frame_and_idk(path: &str, loo: &mut Opti) {
    let mut still = true;
    let mut capt =
        opencv::videoio::VideoCapture::from_file(path, opencv::videoio::CAP_ANY).unwrap();
    let mut frame_count = 0;
    let dir = temporary::Directory::new("frame").unwrap();
    let mut nlop = true;
    let mut img = opencv::prelude::Mat::default();

    while still {
        let read_frame = opencv::prelude::VideoCaptureTrait::read(&mut capt, &mut img).unwrap();
        still = read_frame;
        if still {
            let mut file_name = dir
                .join(&frame_count.to_string())
                .into_os_string()
                .into_string()
                .unwrap();
            file_name.push_str(".png");
            if nlop {
                imgcodecs::imwrite(&file_name, &img, &opencv::core::Vector::default()).unwrap();
            }
            Command::new("feh")
                .arg("--bg-scale")
                .arg(file_name)
                .spawn()
                .unwrap();
            frame_count = frame_count + 1;
        } else {
            if loo == &mut Opti::Loop {
                nlop = false;
                frame_count = 0;
                still = true;
                capt = VideoCapture::from_file(path, opencv::videoio::CAP_ANY).unwrap();
            } else if loo == &mut Opti::Once {
                std::process::exit(1);
            }
        }
    }
    opencv::prelude::VideoCaptureTrait::release(&mut capt).unwrap();
}
