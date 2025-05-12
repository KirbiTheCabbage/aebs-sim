// main.rs
mod traits;
mod sensors;
mod aebs;

use eframe::egui;
use aebs::AebsSystem;
use sensors::LidarSensor;

struct DriverConsole {
    engine_on: bool,
    speed: f32,
    aebs: AebsSystem,

    // Simulation setup (placeholder)
    available_sensors: Vec<String>,
    active_sensors: Vec<String>,
    fault_injected: Option<String>,
    braking_signal: Option<String>,
}

impl Default for DriverConsole {
    fn default() -> Self {
        let mut aebs = AebsSystem::new();
        aebs.add_sensor(Box::new(LidarSensor::new())); // Add 1 sensor to start
        Self {
            engine_on: false,
            speed: 0.0,
            aebs,

            available_sensors: vec![
                "Radar".to_string(),
                "Lidar".to_string(),
                "Camera".to_string(),
                "WheelSpeed".to_string(),
            ],
            active_sensors: vec![],
            fault_injected: None,
            braking_signal: Some("None".to_string()),
        }
    }
}
impl eframe::App for DriverConsole {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Driver Panel");
            ui.separator();

            self.show_engine_controls(ui);
            self.show_speed_controls(ui);
            self.show_aebs_controls(ui);

            ui.separator();
            ui.heading("AEBS Simulation Tools");

            self.show_sensor_management(ui);
            self.show_sensor_data(ui);
            self.show_aebs_state(ui);
            self.show_fault_injection(ui);
        });
    }
}

impl DriverConsole {
    fn show_engine_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui|{
            ui.label(format!("Engine: {}", if self.engine_on { "ON" } else { "OFF" }));
            if ui.button(if self.engine_on { "Stop Engine" } else { "Start Engine" }).clicked() {
                self.engine_on = !self.engine_on;
            }
        });
        ui.horizontal(|ui|{
            ui.label(format!("AEBS: {}", if self.aebs.active { "ON" } else { "OFF" }));
            if ui.button(if self.aebs.active { "Stop Engine" } else { "Start Engine" }).clicked() {
                self.aebs.active = !self.aebs.active;
            }
        });
    }

    fn show_speed_controls(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Speed: {:.1} km/h", self.speed));
        ui.add_enabled_ui(self.engine_on, |ui| {
            ui.add(egui::Slider::new(&mut self.speed, 0.0..=200.0).text("Speed (km/h)"));
        });
    }

    fn show_aebs_controls(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.heading(format!("AEBS : {}", if self.aebs.active { "Active" } else { "OFF" }));
        ui.separator();
        if ui.button(if self.aebs.active { "Disable AEBS" } else { "Enable AEBS" }).clicked() {
            self.aebs.active = !self.aebs.active;
        }

        self.aebs.evaluate();

        ui.label(format!("Brake Level: {}%", self.aebs.brake_level));
        ui.label(format!(
            "Fault Detected: {}",
            if self.aebs.fault_detected { "YES" } else { "NO" }
        ));
    }

    fn show_sensor_management(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                // Left panel: Add Sensor buttons
                ui.vertical(|ui| {
                    ui.label("Sensor Management");

                    // Adding sensors
                    for sensor in &self.available_sensors {
                        if ui.button(format!("Add {}", sensor)).clicked() {
                            if !self.active_sensors.contains(sensor) {
                                self.active_sensors.push(sensor.clone());
                            }
                        }
                    }
                });

                // Right panel: Display active sensors
                ui.vertical(|ui| {
                    ui.label("Active Sensors");

                    if self.active_sensors.is_empty() {
                        ui.label("No active sensors.");
                    } else {
                        // Remove sensors
                        let mut to_remove: Option<String> = None;
                        for sensor in &self.active_sensors {
                            ui.horizontal(|ui| {
                                ui.label(format!("✔️ {}", sensor));
                                if ui.button("Remove").clicked() {
                                    to_remove = Some(sensor.clone());
                                }
                            });
                        }
                        if let Some(sensor) = to_remove {
                            self.active_sensors.retain(|s| s != &sensor);
                        }
                    }
                });
            });
        });
    }


    fn show_sensor_data(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.group(|ui| {
            ui.label("Sensor Data (Stub)");

            if self.active_sensors.is_empty() {
                ui.label("No active sensors.");
            } else {
                for sensor in &self.active_sensors {
                    ui.horizontal(|ui| {
                        ui.label(format!("{sensor}: "));
                        ui.label("Sample data..."); // Placeholder
                    });
                }
            }
        });
    }

    fn show_aebs_state(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.group(|ui| {
            ui.label("AEBS Evaluation");

            ui.label(format!("Braking Signal: {}", self.braking_signal.clone().unwrap_or("Unknown".to_string())));
        });
    }

    fn show_fault_injection(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.group(|ui| {
            ui.label("Fault Injection");

            // Fault Injection for active sensors
            for sensor in &self.active_sensors {
                if ui.button(format!("Inject Fault: {}", sensor)).clicked() {
                    self.fault_injected = Some(sensor.clone());
                }
            }

            // Display current fault
            if let Some(ref fault) = self.fault_injected {
                ui.label(format!("Fault Injected in: {}", fault));
                if ui.button("Clear Fault").clicked() {
                    self.fault_injected = None;
                }
            }
        });
    }
}

// main.rs continued
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Driver Console",
        options,
        Box::new(|_cc| Box::new(DriverConsole::default())),
    )
}
