use std::f64::consts::TAU;
// use std::ops::RangeInclusive;

use egui::plot;
use egui::*;
use plot::{ CoordinatesFormatter, Corner, Legend, Line, LineStyle,  Plot, Value, Values};

// ----------------------------------------------------------------------------

#[derive(PartialEq)]
pub struct LineDemo {
    animate: bool,
    time: f64,
    circle_radius: f64,
    circle_center: Pos2,
    square: bool,
    proportional: bool,
    coordinates: bool,
    line_style: LineStyle,
}

impl Default for LineDemo {
    fn default() -> Self {
        Self {
            animate: !cfg!(debug_assertions),
            time: 0.0,
            circle_radius: 1.5,
            circle_center: Pos2::new(0.0, 0.0),
            square: false,
            proportional: true,
            coordinates: true,
            line_style: LineStyle::Solid,
        }
    }
}

impl LineDemo {
    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            animate,
            time: _,
            circle_radius,
            circle_center,
            square,
            proportional,
            line_style,
            coordinates,
            ..
        } = self;

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label("Circle:");
                    ui.add(
                        egui::DragValue::new(circle_radius)
                            .speed(0.1)
                            .clamp_range(0.0..=f64::INFINITY)
                            .prefix("r: "),
                    );
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut circle_center.x)
                                .speed(0.1)
                                .prefix("x: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut circle_center.y)
                                .speed(1.0)
                                .prefix("y: "),
                        );
                    });
                });
            });
            

            ui.vertical(|ui| {
                ui.style_mut().wrap = Some(false);
                ui.checkbox(animate, "Animate");
                ui.checkbox(square, "Square view")
                    .on_hover_text("Always keep the viewport square.");
                ui.checkbox(proportional, "Proportional data axes")
                    .on_hover_text("Tick are the same size on both axes.");
                ui.checkbox(coordinates, "Show coordinates")
                    .on_hover_text("Can take a custom formatting function.");

                ComboBox::from_label("Line style")
                    .selected_text(line_style.to_string())
                    .show_ui(ui, |ui| {
                        for style in [
                            LineStyle::Solid,
                            LineStyle::dashed_dense(),
                            LineStyle::dashed_loose(),
                            LineStyle::dotted_dense(),
                            LineStyle::dotted_loose(),
                        ]
                        .iter()
                        {
                            ui.selectable_value(line_style, *style, style.to_string());
                        }
                    });
            });
        });
    }

    fn circle(&self) -> Line {
        let n = 512;
        let circle = (0..=n).map(|i| {
            let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
            let r = self.circle_radius;
            Value::new(
                r * t.cos() + self.circle_center.x as f64,
                r * t.sin() + self.circle_center.y as f64,
            )
        });
        Line::new(Values::from_values_iter(circle))
            .color(Color32::from_rgb(100, 200, 100))
            .style(self.line_style)
            .name("circle")
    }

    fn sin(&self) -> Line {
        let time = self.time;
        Line::new(Values::from_explicit_callback(
            move |x| 0.5 * (2.0 * x).sin() * time.sin(),
            ..,
            512,
        ))
        .color(Color32::from_rgb(200, 100, 100))
        .style(self.line_style)
        .name("wave")
    }

    fn thingy(&self) -> Line {
        let time = self.time;
        Line::new(Values::from_parametric_callback(
            move |t| ((2.0 * t + time).sin(), (3.0 * t).sin()),
            0.0..=TAU,
            256,
        ))
        .color(Color32::from_rgb(100, 150, 250))
        .style(self.line_style)
        .name("x = sin(2t), y = sin(3t)")
    }
}

impl LineDemo {
    fn ui(&mut self, ui: &mut Ui) -> Response {
        self.options_ui(ui);
        if self.animate {
            ui.ctx().request_repaint();
            self.time += ui.input().unstable_dt.at_most(1.0 / 30.0) as f64;
        };
        let mut plot = Plot::new("lines_demo").legend(Legend::default());
        if self.square {
            plot = plot.view_aspect(1.0);
        }
        if self.proportional {
            plot = plot.data_aspect(1.0);
        }
        if self.coordinates {
            plot = plot.coordinates_formatter(Corner::LeftBottom, CoordinatesFormatter::default());
        }
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.circle());
            plot_ui.line(self.sin());
            plot_ui.line(self.thingy());
        })
        .response
    }
}


impl eframe::App for LineDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {self.ui(ui)});
        egui::Window::new("My Window").open(&mut true).show(ctx, |ui| {
            ui.label("Hello World!");
         });
    }
}