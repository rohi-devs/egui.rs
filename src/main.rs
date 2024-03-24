
use eframe::{egui, App};


fn main() -> Result<(), eframe::Error> {

    let opt = eframe::NativeOptions {
        viewport : egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };

    eframe::run_native("ROHITH TRAIL",opt, Box::new(|cc|{
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Box::<MyApp>::default()
    }));


    Ok(())
}

struct MyApp {
    name : String,
    age : i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name : "Rohith".to_string(),
            age : 20,
        }
    }
    
}

impl App for MyApp{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");
            ui.label("Name");
            // ui.text_edit_singleline(&mut self.name);
            ui.text_edit_multiline(&mut self.name);
            ui.label("Age");
            ui.add(egui::Slider::new(&mut self.age, 0..=100));
        });
    }
}