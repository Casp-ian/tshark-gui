#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Color32, Shape, Stroke, Ui};

use crate::visualizer::{Pos, Visualizer};

pub fn open_window(visualizer: Visualizer) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 880.0]),
        ..Default::default()
    };

    eframe::run_native(
        "tshark-gui",
        options,
        Box::new(|cc| Ok(Box::<App>::new(App::new(cc, visualizer)))),
    )
}

struct App {
    visualizer: Visualizer,
}

impl App {
    fn new(_cc: &eframe::CreationContext, visualizer: Visualizer) -> Self {
        Self { visualizer }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            draw(ui, ctx, &self.visualizer);
        });
        ctx.request_repaint();
        update_and_solve(&mut self.visualizer);
    }
}

fn update_and_solve(vis: &mut Visualizer) {
    vis.update();
    vis.solve(0.000001);
}

fn draw(ui: &mut Ui, ctx: &egui::Context, vis: &Visualizer) {
    for node in &vis.nodes {
        draw_computer(
            ui,
            node.pos.mul(ctx.viewport_rect().width()),
            &node.computer.ip.to_string(),
        );
    }

    for line in &vis.lines {
        // println!("{:?}", line);
        draw_stroke(
            ui,
            line.from.mul(ctx.viewport_rect().width()),
            line.to.mul(ctx.viewport_rect().width()),
        );
    }
}

fn draw_computer(ui: &mut Ui, pos: Pos, name: &str) {
    ui.painter().text(
        egui::Pos2 { x: pos.0, y: pos.1 },
        // pos.into(),
        egui::Align2::CENTER_CENTER,
        name,
        egui::FontId {
            size: 8.0,
            family: egui::FontFamily::Monospace,
        },
        Color32::WHITE,
    );
}

fn draw_stroke(ui: &mut Ui, from: Pos, to: Pos) {
    ui.painter().line_segment(
        [
            egui::Pos2 {
                x: from.0,
                y: from.1,
            },
            egui::Pos2 { x: to.0, y: to.1 },
        ],
        Stroke {
            width: 5.0,
            color: Color32::RED,
        },
    );
}
