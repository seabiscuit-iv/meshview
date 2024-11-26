
use std::sync::{Arc, Mutex};

use camera::Camera;
use eframe::{egui, egui_glow, glow::{self, HasContext, RIGHT}};
use egui::{mutex, Margin};
use nalgebra::{Matrix3, Orthographic3, Vector3, Vector4};

mod Shader;
use Shader::{Mesh, ShaderProgram};


mod camera;


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
    mesh: Arc<Mutex<Mesh>>,
    camera: Arc<Mutex<Camera>>,
    shader_program: Arc<Mutex<ShaderProgram>>,
    value: f32,
    angle: (f32, f32, f32)
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
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
        });

        egui::TopBottomPanel::bottom("Bottom Panel")
            .frame(egui::Frame { inner_margin: 
                Margin { 
                    left: (10.0), right: (10.0), top: (8.0), bottom: (8.0) 
                }, 
                ..egui::Frame::default()
            })
            .show(ctx, |ui| {

                ui.add_space(4.0);
                ui.label("Camera Position");
                ui.add(egui::DragValue::new(&mut self.camera.lock().unwrap().pos.x));
                ui.add(egui::DragValue::new(&mut self.camera.lock().unwrap().pos.y));
                ui.add(egui::DragValue::new(&mut self.camera.lock().unwrap().pos.z));

                if ctx.input(|i| i.key_down(egui::Key::W)) {
                    ui.label("True");
                } else {
                    ui.label("False");
                }
            });

        //update logic
        let rot = nalgebra::Rotation3::from_euler_angles(
            self.angle.0.to_radians(), 
            self.angle.1.to_radians(), 
            self.angle.2.to_radians()
        );

        if ctx.input(|i| i.key_down(egui::Key::W)) {
            let mut cam = self.camera.lock().unwrap();
            let look = cam.look;
            cam.pos += look * 0.1;
        }
        if ctx.input(|i| i.key_down(egui::Key::S)) {
            let mut cam = self.camera.lock().unwrap();
            let look = cam.look;
            cam.pos += look * -0.1;
        }

        if ctx.input(|i| i.key_down(egui::Key::A)) {
            let mut cam = self.camera.lock().unwrap();
            let right = cam.right;
            cam.pos += right * -0.1;
        }

        if ctx.input(|i| i.key_down(egui::Key::D)) {
            let mut cam = self.camera.lock().unwrap();
            let right = cam.right;
            cam.pos += right * 0.1;
        }

        if ctx.input(|i| i.key_down(egui::Key::Q)) {
            let mut cam = self.camera.lock().unwrap();
            let up = cam.get_up_vec() ;
            cam.pos += up * -0.1;
        }
        
        if ctx.input(|i| i.key_down(egui::Key::E)) {
            let mut cam = self.camera.lock().unwrap();
            let up = cam.get_up_vec() ;
            cam.pos += up * 0.1;
        }


        let look = rot * Vector3::new(0.0, 0.0, -1.0);
        let right = rot * Vector3::new(1.0, 0.0, 0.0);
        self.camera.lock().unwrap().right = right;
        self.camera.lock().unwrap().look = look;

        ctx.request_repaint();
    }
}


impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let mesh = Mesh::new(gl,
            vec![
                Vector3::new(-0.5, -0.5, 0.0),
                Vector3::new(0.5, -0.5, 0.0),
                Vector3::new(0.5, 0.5, 0.0),
                Vector3::new(-0.5, -0.5, 0.0),
                Vector3::new(0.5, 0.5, 0.0),
                Vector3::new(-0.5, 0.5, 0.0),
            ].into_iter().map(|v| {
                v + Vector3::new(0.0, 0.0, 0.0)
            })
            .collect(),
            vec![
                0, 1, 2,
                3, 4, 5
            ]
        );
        let shader_program = ShaderProgram::new(gl, "src/main.vert.glsl", "src/main.frag.glsl");
        
        let camera = Camera::default();
        
        Self { 
            mesh: Arc::new(Mutex::new(mesh)), 
            shader_program: Arc::new(Mutex::new(shader_program)),
            camera: Arc::new(Mutex::new(camera)),
            value: 0.0,
            angle: (0.0, 0.0, 0.0)
        }
    }

    fn custom_painting(&mut self, ui : &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(400.0), egui::Sense::drag());

        let shader_program = self.shader_program.clone();
        let mesh = self.mesh.clone();
        let camera = self.camera.clone();

        self.angle.0 += response.drag_motion().y * -0.1;
        self.angle.1 += response.drag_motion().x * -0.1;

        let value = self.value;

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                unsafe { 
                    painter.gl().use_program(Some((*shader_program.lock().unwrap()).program));
                    painter.gl().uniform_1_f32(
                        Some(&painter.gl().get_uniform_location(shader_program.lock().unwrap().program, "u_Offset").unwrap()), 
                        value
                    );
                };

                shader_program.lock().unwrap().paint(painter.gl(), &mesh.lock().unwrap(), &camera.lock().unwrap());
            })),
        };
        ui.painter().add(callback);
    }
}