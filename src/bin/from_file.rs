use egui::vec2;
use egui::{
    plot::{log_grid_spacer, Line, Plot, PlotImage, PlotPoints, Points},
    Color32, ColorImage, Slider, TextureHandle, TextureOptions, Vec2,
};
use hackathon::detect;

use image::{open, ImageBuffer, Rgb};

fn display(data: Vec<Vec2>, images: Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>) {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "Airplane",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, data, images))),
    )
    .unwrap();
}
fn main() {
    let mut data = vec![];
    let mut images = vec![];
    for i in 400..=600 {
        let img = open(format!("data/frames/{:03}.jpg", i)).unwrap();
        let img_buffer = img.to_rgb8();
        images.push(img_buffer);
    }

    for i in 1..images.len() {
        let x = detect::airplane(&images[0], &images[i]);
        data.push(vec2(x.0 as f32, images[0].height() as f32 - x.1 as f32));
    }
    display(data, images);
}
struct EguiApp {
    cur: usize,
    data: Vec<Vec2>,
    textures: Vec<TextureHandle>,
}

impl EguiApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        data: Vec<Vec2>,
        images: Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    ) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let textures = images
            .iter()
            .enumerate()
            .map(|(i, ib)| {
                cc.egui_ctx.load_texture(
                    format!("data/short/{:03}", i),
                    ColorImage::from_rgb(
                        [ib.width() as usize, ib.height() as usize],
                        ib.as_flat_samples().as_slice(),
                    ),
                    TextureOptions::default(),
                )
            })
            .collect();

        Self {
            data,
            cur: 0,
            textures,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let tex = &self.textures[self.cur];
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
                Slider::new(&mut self.cur, 0..=(self.textures.len() - 2))
                    .text("Number of Frames"),
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
    fn post_rendering(
        &mut self,
        _window_size_px: [u32; 2],
        _frame: &eframe::Frame,
    ) {
    }
}
