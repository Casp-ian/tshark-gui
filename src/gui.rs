#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Color32, Shape, Ui};

use crate::network::Network;
use crate::visualizer::Visualizer;

pub fn open_window(network: Network) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 880.0]),
        ..Default::default()
    };

    eframe::run_native(
        "tshark-gui",
        options,
        Box::new(|cc| Ok(Box::<App>::new(App::new(cc, network)))),
    )
}

struct App {
    network: Network,
    visualizer: Visualizer,
}

impl App {
    fn new(_cc: &eframe::CreationContext, network: Network) -> Self {
        let visualizer = Visualizer::new();
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            network,
            visualizer,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            draw(ui, &self.visualizer);
        });
        ctx.request_repaint();
        self.network.update();
    }
}

fn draw(ui: &mut Ui, vis: &Visualizer) {
    // stroke(ui, 1.0);
    // stroke(ui, 50.0);
    // computer(ui, "test");

    for node in &vis.nodes {
        computer(ui, &node.group);
    }

    for line in &vis.lines {
        stroke(ui, 1.0);
    }
}

fn computer(ui: &mut Ui, name: &str) {
    ui.painter().text(
        egui::Pos2 { x: 200.0, y: 100.0 },
        egui::Align2::CENTER_CENTER,
        name,
        egui::FontId {
            size: 32.0,
            family: egui::FontFamily::Monospace,
        },
        Color32::WHITE,
    );
}

fn stroke(ui: &mut Ui, i: f32) {
    ui.painter().add(Shape::Circle(egui::epaint::CircleShape {
        center: egui::Pos2 {
            x: 100.0,
            y: 100.0 + i,
        },
        radius: 50.0,
        fill: Color32::RED,
        stroke: egui::Stroke {
            width: 0.1,
            color: Color32::BLUE,
        },
    }));
}
