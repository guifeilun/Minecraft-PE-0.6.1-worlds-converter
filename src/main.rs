mod block_map;
mod converter;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use converter::{convert, read_bedrock_level_name, ConvertOptions};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 500.0])
            .with_min_inner_size([480.0, 400.0])
            .with_title("Bedrock to PE 0.6.1 Converter"),
        ..Default::default()
    };

    eframe::run_native(
        "bedrock-to-pe061",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

type SharedString = Arc<Mutex<String>>;
type SharedFloat = Arc<Mutex<f32>>;

struct App {
    input_path: String,
    output_path: String,
    level_name: String,
    infinite: bool,
    convert_overworld: bool,
    convert_nether: bool,
    progress: f32,
    status: String,
    running: Arc<Mutex<bool>>,
    status_arc: Option<SharedString>,
    progress_arc: Option<SharedFloat>,
}

impl App {
    fn new() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            level_name: "World".into(),
            infinite: false,
            convert_overworld: true,
            convert_nether: false,
            progress: 0.0,
            status: "Ready".into(),
            running: Arc::new(Mutex::new(false)),
            status_arc: None,
            progress_arc: None,
        }
    }

    fn start_convert(&mut self) {
        if self.input_path.is_empty() || self.output_path.is_empty() {
            self.status = "Select input and output folders first".into();
            return;
        }
        if !self.convert_overworld && !self.convert_nether {
            self.status = "Select at least one dimension".into();
            return;
        }

        let opts = ConvertOptions {
            input_path: PathBuf::from(&self.input_path),
            output_path: PathBuf::from(&self.output_path),
            infinite: self.infinite,
            convert_overworld: self.convert_overworld,
            convert_nether: self.convert_nether,
            level_name: self.level_name.clone(),
        };

        let running = self.running.clone();
        *running.lock().unwrap() = true;

        let status_arc: SharedString = Arc::new(Mutex::new("Starting...".into()));
        let progress_arc: SharedFloat = Arc::new(Mutex::new(0.0f32));

        let status_clone = status_arc.clone();
        let status_clone2 = status_arc.clone();
        let progress_clone = progress_arc.clone();

        std::thread::spawn(move || {
            let progress_fn: converter::ProgressFn = Arc::new(move |p, s| {
                *progress_clone.lock().unwrap() = p;
                *status_clone.lock().unwrap() = s.to_string();
            });

            match convert(&opts, progress_fn) {
                Ok(_) => { *status_clone2.lock().unwrap() = "Done".into(); }
                Err(e) => { *status_clone2.lock().unwrap() = format!("Failed: {}", e); }
            }

            *running.lock().unwrap() = false;
        });

        self.status_arc = Some(status_arc);
        self.progress_arc = Some(progress_arc);
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Bedrock to PE 0.6.1 Converter");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Input (Bedrock db folder):");
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.input_path = path.display().to_string();
                    if let Some(name) = read_bedrock_level_name(&PathBuf::from(&self.input_path)) {
                        self.level_name = name;
                    }
                }
            }
        });
        ui.label(&self.input_path);

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Output (0.6.1 world folder):");
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.output_path = path.display().to_string();
                }
            }
        });
        ui.label(&self.output_path);

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("World name:");
            ui.text_edit_singleline(&mut self.level_name);
        });

        ui.add_space(8.0);
        ui.separator();

        ui.label("World type:");
        ui.radio_value(&mut self.infinite, false, "Finite (256x256)");
        ui.radio_value(&mut self.infinite, true, "Infinite");

        ui.add_space(8.0);
        ui.separator();

        ui.label("Dimensions:");
        ui.checkbox(&mut self.convert_overworld, "Overworld");
        ui.checkbox(&mut self.convert_nether, "Nether");

        ui.add_space(8.0);
        ui.separator();

        let is_running = *self.running.lock().unwrap();
        if ui.add_enabled(!is_running, egui::Button::new("Start Convert")).clicked() {
            self.start_convert();
        }

        ui.add_space(8.0);
        ui.add(egui::ProgressBar::new(self.progress).show_percentage());
        ui.label(&self.status);

        if let Some(arc) = &self.status_arc {
            if let Ok(s) = arc.lock() {
                self.status = s.clone();
            }
        }
        if let Some(arc) = &self.progress_arc {
            if let Ok(p) = arc.lock() {
                self.progress = *p;
            }
        }

        ui.ctx().request_repaint();
    }
}