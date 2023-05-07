use std::sync::Arc;

use egui::{
    plot::{log_grid_spacer, Line, Plot, PlotImage, PlotPoints, Points},
    Color32, ColorImage, Vec2,
};
use egui_extras::RetainedImage;

use crate::data_share::DataTransfer;

pub fn display(data_transfer: Arc<DataTransfer>) {
    let native_options = eframe::NativeOptions {
        fullscreen: false,
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
    texture: RetainedImage,
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
        //_cc.egui_ctx.set_visuals(Visuals::light());
        let transfer_data = data_transfer.val.lock().unwrap();
        let texture = egui_extras::RetainedImage::from_color_image(
            "tex",
            ColorImage::from_rgb(
                transfer_data.image_size,
                &transfer_data.image,
            ),
        );
        drop(transfer_data);
        Self {
            texture,
            data_transfer,
        }
    }
}
const MAX_NUM_DATA: usize = 100;

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut transfer_data = self.data_transfer.val.lock().unwrap();
        println!("DIGGA");
        transfer_data = self.data_transfer.cv.wait(transfer_data).unwrap();
        let retained_image = egui_extras::RetainedImage::from_color_image(
            "tex",
            ColorImage::from_rgb(
                transfer_data.image_size,
                &transfer_data.image,
            ),
        );
        self.texture = retained_image;
        if transfer_data.data_points.len() >= MAX_NUM_DATA {
            transfer_data.data_points.remove(0);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            //UPDATE
            let plot_image = PlotImage::new(
                self.texture.texture_id(ctx),
                egui::plot::PlotPoint {
                    x: self.texture.width() as f64 / 2.,
                    y: self.texture.height() as f64 / 2.,
                },
                Vec2::new(
                    self.texture.width() as f32,
                    self.texture.height() as f32,
                ),
            )
            .tint(Color32::from_white_alpha(32));
            let plot_points: PlotPoints = transfer_data
                .data_points
                .iter()
                .map(|v| [v.0 as f64, v.1 as f64])
                .collect();
            let points = Points::new(plot_points).radius(4.);
            let plot_points: PlotPoints = transfer_data
                .data_points
                .iter()
                .map(|v| [v.0 as f64, v.1 as f64])
                .collect();
            let line = Line::new(plot_points).width(2.);
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
            ctx.request_repaint();
            self.data_transfer.cv.notify_all();
        });
    }
}
