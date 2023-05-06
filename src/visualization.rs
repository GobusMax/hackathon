use egui::{
    plot::{log_grid_spacer, Line, Plot, PlotImage, PlotPoints, Points},
    Color32, ColorImage, Slider, TextureHandle, TextureOptions, Vec2,
};

use image::DynamicImage;

pub struct EguiApp {
    cur: usize,
    data: Vec<Vec2>,
    textures: Vec<TextureHandle>,
}

impl EguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            data: vec![],
            cur: 0,
            textures: vec![],
        }
    }
}
const MAX_NUM_DATA: usize = 100;

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let new_image = getImage();
        self.textures.push(ctx.load_texture(
            "tex",
            ColorImage::from_rgb(
                [new_image.width() as usize, new_image.width() as usize],
                new_image.as_rgb8().unwrap().as_flat_samples().as_slice(),
            ),
            TextureOptions::default(),
        ));
        self.data.push(getPos());
        if self.data.len() >= MAX_NUM_DATA {
            self.data.remove(0);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            //UPDATE
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
}

fn getImage() -> DynamicImage {
    todo!()
}
fn getPos() -> Vec2 {
    todo!()
}
