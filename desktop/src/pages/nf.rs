use std::fs::{self, remove_dir_all};
use ureq::Error::Status;

use directories::BaseDirs;
use eframe::{
    egui::{CentralPanel, Layout, RichText, ScrollArea, Separator, TextEdit, TextStyle::Monospace},
    App,
};
use egui_extras::RetainedImage;
use ureq_multipart::MultipartBuilder;

use crate::structures::{err_win, AccInfo, ImgBytes, NfPage, PostResp, PADDING, WARN, WHITE};

impl App for NfPage {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
        if ri.authd {
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("Tracebook"));
                ui.add_space(5.);
                ui.add(Separator::default().spacing(20.));
                ui.with_layout(Layout::left_to_right(eframe::emath::Align::Min), |ui| {
                    ui.add(
                        TextEdit::multiline(&mut self.caption)
                            .font(Monospace)
                            .desired_rows(1),
                    );
                    if ui.button("Image").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.image = Some(path.display().to_string())
                        }
                    }

                    if !(self.image.clone().unwrap_or("".into()).is_empty()) {
                        if infer::is_image(&fs::read(self.image.clone().unwrap()).unwrap()) {
                            self.type_supt = Some(true);
                        } else {
                            self.type_supt = Some(false);
                        }
                    }

                    let post_but = ui.button("Post");

                    if let Some(sup) = self.type_supt {
                        if !sup {
                            ui.colored_label(
                                WARN,
                                RichText::new("File Type Not Support!").text_style(Monospace),
                            );
                        }
                    }

                    if post_but.clicked() {
                        if self.image.clone().unwrap_or("".into()).is_empty() {
                            (self.cont_type, self.req_data) = MultipartBuilder::new()
                                .add_text("token", &ri.token)
                                .unwrap()
                                .add_text("caption", &self.caption)
                                .unwrap()
                                .finish()
                                .unwrap();
                        } else {
                            (self.cont_type, self.req_data) = MultipartBuilder::new()
                                .add_file("file", self.image.clone().unwrap())
                                .unwrap()
                                .add_text("token", &ri.token)
                                .unwrap()
                                .add_text("caption", &self.caption)
                                .unwrap()
                                .finish()
                                .unwrap();
                        }
                        let resp = ureq::post("http://localhost:8090/post/upload_post")
                            .set("Content-Type", &self.cont_type)
                            .send_bytes(&self.req_data);

                        match resp {
                            Ok(_) => {}
                            Err(Status(404, _)) => err_win(
                                ctx,
                                &mut self.err_win,
                                "Sorry You're offline or server is offline!",
                            ),
                            Err(Status(406, _)) => err_win(
                                ctx,
                                &mut self.err_win,
                                "Sorry There is no token Relogin and try again!",
                            ),
                                Err(_) => err_win(ctx, &mut self.err_win, "Sorry Something went wrong Please Relogin and try again or Report to Developer Team!")
                        }

                        self.image = None;
                        self.caption = "".into();
                    }
                });
                ScrollArea::new([true, false]).show(ui, |ui| {
                    match ureq::get("http://localhost:8090/fetch_post").call() {
                        Ok(js_req) => match js_req.into_json::<PostResp>() {
                            Ok(req) => {
                                for i in req.val {
                                    ui.add_space(PADDING);
                                    let fullname = RichText::new(format!(
                                        "💀 {}",
                                        i.fullname[1..i.fullname.len() - 1].to_string()
                                    ))
                                    .text_style(Monospace)
                                    .size(15.);
                                    ui.colored_label(WHITE, fullname);
                                    ui.label(
                                        RichText::new(format!(
                                            "@{}",
                                            i.owner[5..i.owner.len()].to_string()
                                        ))
                                        .size(8.),
                                    );
                                    if !i.caption.is_empty() {
                                        ui.colored_label(
                                            WHITE,
                                            RichText::new(i.caption).size(13.5),
                                        );
                                    }
                                    if !i.image.is_empty() {
                                        let bytes = ureq::get(&format!(
                                            "http://localhost:8090/images/{}",
                                            i.image
                                        ))
                                        .call();
                                        match bytes {
                                            Ok(img_bytes) => {
                                                match img_bytes.into_json::<ImgBytes>() { Ok(img_bytess) =>{
                                        let image_retain = RetainedImage::from_image_bytes(
                                            format!("{}", i.image),
                                            &img_bytess.value,
                                        );
                                        match image_retain {Ok(image_img) => {
                                        image_img.show_size(
                                            ui,
                                            eframe::epaint::Vec2 { x: 225., y: 225. },
                                        );},
                                                    Err(_) => err_win(ctx, &mut self.err_win, "Error While Making RetainedImage Report to Developer Team!")}}
                                            Err(_) => err_win(ctx, &mut self.err_win, "I don't know what the fuck is wrong man just report!")
                                                }
                                            }
                                            Err(Status(404, _)) => err_win(ctx, &mut self.err_win, "Image not found or You're offline or server is offline!"),
                                            Err(_) => err_win(ctx, &mut self.err_win, "Something went wrong Please try again and report to Developer Team!")
                                        }
                                    }
                                }
                            }
                            Err(_) => err_win(
                                ctx,
                                &mut self.err_win,
                                "Error in Parsing Json Please Report to Developer Team!",
                            ),
                        },
                        Err(Status(404, _)) => err_win(
                            ctx,
                            &mut self.err_win,
                            "Sorry You're offline or server is offline!",
                        ),
                        Err(_) => err_win(
                            ctx,
                            &mut self.err_win,
                            "Sorry Something went wrong Please Report to Developer Team!",
                        ),
                    }
                });
            });
        }
    }
}
