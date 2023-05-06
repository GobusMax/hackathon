use egui::{
    plot::{log_grid_spacer, Line, Plot, PlotImage, PlotPoints, Points},
    Color32, ColorImage, Slider, TextureHandle, TextureOptions, Vec2,
};

use image::{open, DynamicImage, ImageBuffer, Rgb};

pub struct EguiApp {
    tex: TextureHandle,
    data: Vec<Vec2>,
    cur: usize,
    images: Vec<TextureHandle>,
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
        let tex = cc.egui_ctx.load_texture(
            "Background Texture",
            ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()),
            TextureOptions::default(),
        );
        let mut images = vec![];
        for i in 1..=52 {
            let img = open(format!("data/short/{:03}.png", i)).unwrap();
            let s = [img.width() as _, img.height() as _];
            let img_buffer = img.to_rgba8();
            let pixs = img_buffer.as_flat_samples();

            images.push(cc.egui_ctx.load_texture(
                format!("data/short/{:03}", i),
                ColorImage::from_rgba_unmultiplied(s, pixs.as_slice()),
                TextureOptions::default(),
            ));
        }
        Self {
            tex,
            data,
            cur: 0,
            images,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let tex = &self.images[self.cur + 1];
            let plot_image = PlotImage::new(
                tex.id(),
                egui::plot::PlotPoint {
                    x: tex.size()[0] as f64 / 2.,
                    y: tex.size()[1] as f64 / 2.,
                },
                tex.size_vec2(),
            )
            .tint(Color32::from_white_alpha(32));
            let plot_points: PlotPoints = self.data[0..self.cur]
                .iter()
                .map(|v| [v.x as f64, v.y as f64])
                .collect();
            let points = Points::new(plot_points).radius(4.);
            let plot_points: PlotPoints = self.data[0..self.cur]
                .iter()
                .map(|v| [v.x as f64, v.y as f64])
                .collect();
            let line = Line::new(plot_points).width(2.);
            ui.add(
                Slider::new(&mut self.cur, 0..=(self.images.len() - 2))
                    .text("Test"),
            );
            Plot::new("Plot")
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
