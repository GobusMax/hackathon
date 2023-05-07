use std::sync::Arc;

use egui::{
    plot::{log_grid_spacer, Line, Plot, PlotImage, PlotPoints, Points},
    Color32, ColorImage, Slider, Visuals,
};
use egui_extras::RetainedImage;

use crate::data_share::DataTransfer;

pub fn display(data_transfer: Arc<DataTransfer>) {
    let native_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };
    eframe::run_native(
        "Airplane",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc, data_transfer))),
    )
    .unwrap();
}

pub struct EguiApp {
    cur: usize,
    textures: Vec<RetainedImage>,
    data_transfer: Arc<DataTransfer>,
}

impl EguiApp {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        data_transfer: Arc<DataTransfer>,
    ) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        _cc.egui_ctx.set_visuals(Visuals::light());
        Self {
            cur: 0,
            textures: vec![],
            data_transfer,
        }
    }
}
const MAX_NUM_DATA: usize = 100;

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut transfer_data = self.data_transfer.val.lock().unwrap();
        let retained_image = egui_extras::RetainedImage::from_color_image(
            "tex",
            ColorImage::from_rgb(
                transfer_data.image_size,
                &transfer_data.image,
            ),
        );
        self.textures.push(retained_image);
        if transfer_data.data_points.len() >= MAX_NUM_DATA {
            transfer_data.data_points.remove(0);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            //UPDATE
            let tex = &self.textures[self.cur];
            let plot_image = PlotImage::new(
                tex.texture_id(ctx),
                egui::plot::PlotPoint {
                    x: tex.size()[0] as f64 / 2.,
                    y: tex.size()[1] as f64 / 2.,
                },
                tex.size_vec2(),
            )
            .tint(Color32::from_white_alpha(32));
            let plot_points: PlotPoints = transfer_data.data_points
                [0..self.cur]
                .iter()
                .map(|v| [v.0 as f64, v.1 as f64])
                .collect();
            let points = Points::new(plot_points).radius(4.);
            let plot_points: PlotPoints = transfer_data.data_points
                [0..self.cur]
                .iter()
                .map(|v| [v.0 as f64, v.1 as f64])
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
