use std::default;

use eframe::{egui, egui_glow, glow};
use egui::Margin;


fn main() -> eframe::Result{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]).with_position([100.0, 100.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "MeshView",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}


// Main App UI

struct App {
    
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top Panel")
            .frame(egui::Frame { inner_margin: 
                Margin { 
                    left: (10.0), right: (10.0), top: (8.0), bottom: (8.0) 
                }, 
                ..egui::Frame::default()
            })
            .show(ctx, |ui| {
                ui.button("Top Panel")
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.button("Image")
        });
    }
}


impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self{}
    }
}