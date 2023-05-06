use egui::{
    plot::{Line, Plot, PlotImage, PlotPoints},
    ColorImage, TextureHandle, TextureOptions, Vec2,
};

use image::{io::Reader, DynamicImage};

fn main() {
    let img = Reader::open("data/plane.jpg").unwrap().decode().unwrap();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc, img))),
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
            ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
            TextureOptions::default(),
        );
        Self { tex: res }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            let sin: PlotPoints = (0..1000)
                .map(|i| {
                    let x = i as f64 * 0.01;
                    [x, x.sin()]
                })
                .collect();
            let line = Line::new(sin);
            let plot_image = PlotImage::new(
                self.tex.id(),
                egui::plot::PlotPoint { x: 0., y: 0. },
                Vec2::new(self.tex.aspect_ratio(), 1.),
            );
            Plot::new("my_plot").view_aspect(1.0).data_aspect(1.).show(
                ui,
                |plot_ui| {
                    plot_ui.image(plot_image);
                    plot_ui.line(line)
                },
            );
        });
    }
}
