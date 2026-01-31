//! Service factice : Calculatrice (MVP).

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;

/// État interne de la calculatrice.
pub struct CalculatorService {
    display: String,
    current: f64,
    pending_op: Option<CalcOp>,
    reset_display: bool,
}

#[derive(Clone, Copy)]
enum CalcOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl Default for CalculatorService {
    fn default() -> Self {
        Self {
            display: "0".to_string(),
            current: 0.0,
            pending_op: None,
            reset_display: false,
        }
    }
}

impl CalculatorService {
    fn append_digit(&mut self, d: &str) {
        if self.reset_display {
            self.display = d.to_string();
            self.reset_display = false;
        } else if self.display == "0" && d != "." {
            self.display = d.to_string();
        } else {
            self.display.push_str(d);
        }
    }

    fn apply_pending(&mut self) {
        if let Some(op) = self.pending_op {
            let value: f64 = self.display.replace(',', ".").parse().unwrap_or(0.0);
            self.current = match op {
                CalcOp::Add => self.current + value,
                CalcOp::Sub => self.current - value,
                CalcOp::Mul => self.current * value,
                CalcOp::Div => {
                    if value != 0.0 {
                        self.current / value
                    } else {
                        self.current
                    }
                }
            };
            self.display = format_number(self.current);
            self.pending_op = None;
            self.reset_display = true;
        }
    }
}

impl ServiceUi for CalculatorService {
    fn id(&self) -> ServiceId {
        ServiceId::Calculator
    }
    fn title(&self) -> &'static str {
        "Calculatrice"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.heading("Calculatrice");
            ui.add_space(12.0);

            ui.add(
                egui::TextEdit::singleline(&mut self.display)
                    .desired_width(220.0)
                    .font(egui::TextStyle::Monospace.resolve(ui.style()))
                    .horizontal_align(egui::Align::RIGHT),
            );
            ui.add_space(8.0);

            // Ligne 1: 7 8 9 /
            ui.horizontal(|ui| {
                if ui.button("7").clicked() {
                    self.append_digit("7");
                }
                if ui.button("8").clicked() {
                    self.append_digit("8");
                }
                if ui.button("9").clicked() {
                    self.append_digit("9");
                }
                if ui.button("/").clicked() {
                    let v: f64 = self.display.replace(',', ".").parse().unwrap_or(0.0);
                    self.apply_pending();
                    self.current = v;
                    self.pending_op = Some(CalcOp::Div);
                    self.reset_display = true;
                }
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("4").clicked() {
                    self.append_digit("4");
                }
                if ui.button("5").clicked() {
                    self.append_digit("5");
                }
                if ui.button("6").clicked() {
                    self.append_digit("6");
                }
                if ui.button("×").clicked() {
                    let v: f64 = self.display.replace(',', ".").parse().unwrap_or(0.0);
                    self.apply_pending();
                    self.current = v;
                    self.pending_op = Some(CalcOp::Mul);
                    self.reset_display = true;
                }
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("1").clicked() {
                    self.append_digit("1");
                }
                if ui.button("2").clicked() {
                    self.append_digit("2");
                }
                if ui.button("3").clicked() {
                    self.append_digit("3");
                }
                if ui.button("−").clicked() {
                    let v: f64 = self.display.replace(',', ".").parse().unwrap_or(0.0);
                    self.apply_pending();
                    self.current = v;
                    self.pending_op = Some(CalcOp::Sub);
                    self.reset_display = true;
                }
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("C").clicked() {
                    self.display = "0".to_string();
                    self.current = 0.0;
                    self.pending_op = None;
                    self.reset_display = false;
                }
                if ui.button("0").clicked() {
                    self.append_digit("0");
                }
                if ui.button(".").clicked() {
                    if !self.display.contains('.') {
                        self.append_digit(".");
                    }
                }
                if ui.button("+").clicked() {
                    let v: f64 = self.display.replace(',', ".").parse().unwrap_or(0.0);
                    self.apply_pending();
                    self.current = v;
                    self.pending_op = Some(CalcOp::Add);
                    self.reset_display = true;
                }
            });
            ui.add_space(4.0);
            if ui.button("=").clicked() {
                self.apply_pending();
            }
        });
    }
}

fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{:.6}", n).trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
