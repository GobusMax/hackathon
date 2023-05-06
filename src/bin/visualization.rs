use egui::{
    plot::{log_grid_spacer, Line, Plot, PlotImage, PlotPoints, Points},
    vec2, Color32, ColorImage, TextureHandle, TextureOptions, Vec2,
};

use image::{io::Reader, DynamicImage};

fn main() {
    let img = Reader::open("data/plane.jpg").unwrap().decode().unwrap();
    let data = vec![vec2(100., 200.), vec2(231., 364.), vec2(300., 100.)];
    let mut native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    eframe::run_native(
        "Hackathon",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, img, data))),
    )
    .unwrap();
}

pub struct EguiApp {
    tex: TextureHandle,
    data: Vec<Vec2>,
}

impl EguiApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        image: DynamicImage,
        data: Vec<Vec2>,
    ) -> Self {
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
        Self { tex: res, data }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot_image = PlotImage::new(
                self.tex.id(),
                egui::plot::PlotPoint {
                    x: self.tex.size()[0] as f64 / 2.,
                    y: self.tex.size()[1] as f64 / 2.,
                },
                self.tex.size_vec2(),
            )
            .tint(Color32::from_white_alpha(32));
            let plot_points: PlotPoints =
                self.data.iter().map(|v| [v.x as f64, v.y as f64]).collect();
            let points = Points::new(plot_points).radius(4.);
            let plot_points: PlotPoints =
                self.data.iter().map(|v| [v.x as f64, v.y as f64]).collect();
            let line = Line::new(plot_points).width(2.);
            Plot::new("my_plot")
                .view_aspect(1.0)
                .data_aspect(1.)
                .x_grid_spacer(log_grid_spacer(100))
                .y_grid_spacer(log_grid_spacer(100))
                .show(ui, |plot_ui| {
                    plot_ui.image(plot_image);
                    plot_ui.line(line);
                    plot_ui.points(points);
                });
        });
    }
}
