use eframe::egui;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Instant;

struct App {
    file_path: Option<PathBuf>,
    credits: i32,
    hull_strength: f32,
    shields: f32,
    radiation_shields: f32,
    cargo: [i32; 19],
    ripper_rounds: i32,
    depth_charges: i32,
    validation_msg: String,
    save_message_time: Option<Instant>,
    file_dialog_open: bool,
    file_receiver: Option<mpsc::Receiver<Option<PathBuf>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            file_path: None,
            credits: 0,
            hull_strength: 0.0,
            shields: 0.0,
            radiation_shields: 0.0,
            cargo: [0; 19],
            ripper_rounds: 0,
            depth_charges: 0,
            validation_msg: String::new(),
            save_message_time: None,
            file_dialog_open: false,
            file_receiver: None,
        }
    }
}

impl App {
    fn load_file(&mut self, path: PathBuf) {
        if let Ok(data) = fs::read(&path) {
            if data.len() >= 0xAB {
                // Ship parameters
                self.credits = i32::from_le_bytes(data[0x34..0x38].try_into().unwrap());
                self.hull_strength = f32::from_le_bytes(data[0x40..0x44].try_into().unwrap());
                self.radiation_shields = f32::from_le_bytes(data[0x44..0x48].try_into().unwrap());
                self.shields = f32::from_le_bytes(data[0x38..0x3C].try_into().unwrap()) * 10.0;

                // Weapon ammo
                self.ripper_rounds = i32::from_le_bytes(data[0x71B0..0x71B4].try_into().unwrap());
                self.depth_charges = i32::from_le_bytes(data[0x7190..0x7194].try_into().unwrap());
                
                // Cargo
                let cargo_base = 0x4E20;
                for i in 0..19 {
                    let off = cargo_base + i * 4;
                    self.cargo[i] = i32::from_le_bytes(data[off..off + 4].try_into().unwrap());
                }
                
                self.file_path = Some(path);
                self.validation_msg.clear();
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(ref path) = self.file_path {
            if let Ok(mut data) = fs::read(path) {
                data[0x34..0x38].copy_from_slice(&self.credits.to_le_bytes());
                data[0x40..0x44].copy_from_slice(&self.hull_strength.to_le_bytes());
                data[0x44..0x48].copy_from_slice(&self.radiation_shields.to_le_bytes());
                data[0x38..0x3C].copy_from_slice(&(self.shields / 10.0).to_le_bytes());
                data[0x71B0..0x71B4].copy_from_slice(&self.ripper_rounds.to_le_bytes());
                data[0x7190..0x7194].copy_from_slice(&self.depth_charges.to_le_bytes());
                let cargo_base = 0x4E20;
                for i in 0..19 {
                    let off = cargo_base + i * 4;
                    data[off..off + 4].copy_from_slice(&self.cargo[i].to_le_bytes());
                }
                if fs::write(path, data).is_ok() {
                    self.save_message_time = Some(Instant::now());
                }
            }
        }
    }

    fn validate_credits(&mut self) {
        self.validation_msg = if self.credits < 0 {
            "It'd be better for you to provide a value that's >0 ...".to_string()
        } else if self.credits > 1_000_000 {
            "Sub Culture can't handle cash-on-hand amounts larger than 1000000!".to_string()
        } else {
            String::new()
        };
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(receiver) = &self.file_receiver {
            if let Ok(path_opt) = receiver.try_recv() {
                self.file_dialog_open = false;
                self.file_receiver = None;
                if let Some(path) = path_opt {
                    self.load_file(path);
                }
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Subculture Save Editor");
            ui.add_space(20.0);
            
            // Check for Ctrl+L to open file dialog
            if ctx.input(|i| i.key_pressed(egui::Key::L) && i.modifiers.ctrl) && !self.file_dialog_open {
                self.file_dialog_open = true;
                let (sender, receiver) = mpsc::channel();
                self.file_receiver = Some(receiver);
                std::thread::spawn(move || {
                    let result = rfd::FileDialog::new().pick_file();
                    let _ = sender.send(result);
                });
            }
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Credits: ");
                    if ui.add(egui::DragValue::new(&mut self.credits).speed(100)).changed() {
                        self.validate_credits();
                    }
                });
                
                ui.separator();
                ui.label(egui::RichText::new("Ship Parameters").strong().size(16.0));
                ui.horizontal(|ui| {
                    ui.label("Hull Strength: ");
                    ui.add(egui::DragValue::new(&mut self.hull_strength));
                });
                ui.horizontal(|ui| {
                    ui.label("Shields: ");
                    ui.add(egui::DragValue::new(&mut self.shields));
                });
                ui.horizontal(|ui| {
                    ui.label("Radiation Shields: ");
                    ui.add(egui::DragValue::new(&mut self.radiation_shields).speed(0.1));
                });
                
                ui.separator();
                ui.label(egui::RichText::new("Weapon Ammo").strong().size(16.0));
                ui.horizontal(|ui| {
                    ui.label("Ripper Rounds: ");
                    ui.add(egui::DragValue::new(&mut self.ripper_rounds));
                });
                ui.horizontal(|ui| {
                    ui.label("Depth Charges: ");
                    ui.add(egui::DragValue::new(&mut self.depth_charges));
                });
                
                ui.separator();
                ui.label(egui::RichText::new("Cargo").strong().size(16.0));

                let cargo_names = [
                    "Thorium",
                    "Refined Thorium",
                    "Metal",
                    "Processed Metal",
                    "Pearls",
                    "Copper",
                    "Treated Copper",
                    "Oxygen",
                    "Purified Water",
                    "Sea Weed",
                    "Kelp Beer",
                    "Plankton",
                    "Caviar",
                    "Medical Supplies",
                    "Methane",
                    "Tobacco",
                    "Rubber",
                    "Cod Oil",
                    "Lionfish Venom",
                ];

                for (i, name) in cargo_names.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", name));
                        ui.add(egui::DragValue::new(&mut self.cargo[i]));
                    });
                }
            });
            
            ui.separator();
            
            if !self.validation_msg.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.validation_msg);
            }
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    let button_text = if self.file_dialog_open { "Opening file..." } else { "Load File (Ctrl+L)" };
                    if ui.add_enabled(!self.file_dialog_open, egui::Button::new(button_text)).clicked() {
                        self.file_dialog_open = true;
                        let (sender, receiver) = mpsc::channel();
                        self.file_receiver = Some(receiver);
                        std::thread::spawn(move || {
                            let result = rfd::FileDialog::new().pick_file();
                            let _ = sender.send(result);
                        });
                    }
                    
                    let save_enabled = self.file_path.is_some() && self.validation_msg.is_empty();
                    if ui.add_enabled(save_enabled, egui::Button::new("Save (Ctrl+S)")).clicked() || (save_enabled && ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl)) {
                        self.save_file();
                    }
                });
                
                if let Some(save_time) = self.save_message_time {
                    let elapsed = save_time.elapsed().as_secs_f32();
                    if elapsed < 3.0 {
                        let alpha = (1.0 - elapsed / 3.0).max(0.0);
                        let color = egui::Color32::from_rgba_unmultiplied(0, 150, 0, (255.0 * alpha) as u8);
                        ui.colored_label(color, "Changes saved. Enjoy!");
                        ctx.request_repaint();
                    } else {
                        self.save_message_time = None;
                    }
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.viewport.resizable = Some(true);
    options.viewport.inner_size = Some(egui::Vec2::new(400.0, 700.0));
    
    eframe::run_native(
        "Subculture Save Editor",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
