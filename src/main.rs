
use eframe::{egui, App};


fn main() -> Result<(), eframe::Error> {

    let opt = eframe::NativeOptions::default();

    eframe::run_native("ROHITH TRAIL",opt, Box::new(|cc|{
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Box::<MyApp>::default()
    }));


    Ok(())
}

struct MyApp {
    name : String,
    age : i32,
    url : String,
    response : Option<String>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name : "Rohith".to_string(),
            age : 20,
            url : "https://www.google.com".to_string(),
            response : None
        }
    }
    
}

impl App for MyApp{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");
            ui.label("Name");
            ui.text_edit_multiline(&mut self.url);
            let _r = ui.button("Click me");
            if _r.clicked() {
                if let url = &self.url {
                    let req = reqwest::get(url);
                    let _rt = tokio::runtime::Runtime::new().unwrap();
                    let _res = _rt.block_on(req).unwrap();
                    let res = _rt.block_on(_res.text()).unwrap();
                    self.response = Some(res);
                }
            }
            ui.text_edit_multiline(&mut self.name);
            ui.label("Age");
            ui.add(egui::Slider::new(&mut self.age, 0..=100));
            if let Some(mut res) = Some(self.response.clone()) {
                match res {
                    Some(mut r) => {
                        ui.label("Response");
                        ui.text_edit_multiline(&mut r);
                    },
                    None => {}
                    
                }
            }
        });
    }
}