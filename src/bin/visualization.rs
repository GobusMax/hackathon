use egui::{ColorImage, TextureHandle, TextureOptions};

use image::{io::Reader, DynamicImage};

fn main() {
    let img = Reader::open("data/plane.jpg").unwrap().decode().unwrap();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(
            |cc| {
                Box::new(
                    MyEguiApp::new(
                        cc, img,
                    ),
                )
            },
        ),
    )
    .unwrap();
}

struct MyEguiApp {
    tex: TextureHandle,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>, image: DynamicImage) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let res = cc.egui_ctx.load_texture(
            "Blub",
            ColorImage::from_rgba_unmultiplied(
                size,
                pixels.as_slice(),
            ),
            TextureOptions::default(),
        );
        Self { tex: res }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                ui.heading("Hello World!");
                ui.image(
                    self.tex.id(),
                    self.tex.size_vec2(),
                )
            },
        );
    }
}
