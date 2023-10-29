#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle};
use log::{debug, error};

fn set_fonts(ctx: &egui::Context) {
    debug!("Load fonts");

    let mut fonts = FontDefinitions::default();
    // let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "IBM Plex Sans".to_owned(),
        FontData::from_static(include_bytes!("../fonts/IBMPlexSans-Regular.ttf")),
    );

    // match fonts.families.get_mut(&FontFamily::Proportional) {
    //     Some(font_families) => font_families.push("IBM Plex Sans".to_owned()),
    //     None => error!("Didn't find font family. That should not happen.")
    // }

    fonts.families.insert(FontFamily::Proportional, vec!["IBM Plex Sans".into()]);

    ctx.set_fonts(fonts);

    // let mut style = (*ctx.style()).clone();
    // style.text_styles = [
    //     (TextStyle::Body, FontId::new(21.0, FontFamily::Proportional)),
    //     (TextStyle::Button, FontId::new(18.0, FontFamily::Monospace)),
    //     (TextStyle::Monospace, FontId::new(18.0, FontFamily::Monospace)),
    //     (TextStyle::Small, FontId::new(16.0, FontFamily::Proportional)),
    //     (TextStyle::Heading, FontId::new(48.0, FontFamily::Proportional)),
    // ].into();
    // ctx.set_style(style);
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            set_fonts(&cc.egui_ctx);

            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // TODO receive image from somewhere and update it
            let image: Vec<u8> = vec![0, 0, 0, 0];

            // Ok(egui::ColorImage::from_rgb(
            //     [120, 120],
            //     &image
            // ));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}