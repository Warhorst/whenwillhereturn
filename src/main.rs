use chrono::{Local, TimeZone};
use eframe::{App, Frame};
use eframe::epaint::{FontFamily, FontId};
use egui::{CentralPanel, Context, TextStyle};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "When will he return?",
        eframe::NativeOptions::default(),
        Box::new(|_| {
            Box::<WhenWillHeReturnApp>::new(WhenWillHeReturnApp)
        })
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                eframe::WebOptions::default(),
                Box::new(|_| {
                    Box::<WhenWillHeReturnApp>::new(WhenWillHeReturnApp)
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct WhenWillHeReturnApp;

impl App for WhenWillHeReturnApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles.insert(
                TextStyle::Heading,
                FontId::new(36.0, FontFamily::Proportional),
            );

            let return_date = Local.with_ymd_and_hms(2024, 10, 1, 9, 30, 0).unwrap();
            let now = Local::now();
            let delta = return_date.signed_duration_since(now);

            // Yeah, interactive. I am definitely not too lazy to rewrite this in a UI with continuous updates.
            ui.heading("When will he return? This is an interactive counter, so move your mouse to find out!");

            if delta.num_seconds() < 0 {
                ui.heading("He returned!");
            } else {
                let days = delta.num_days();
                let hours = delta.num_hours() % 24;
                let minutes = delta.num_minutes() % 60;
                let seconds = delta.num_seconds() % 60;

                ui.heading(format!("He will return in {days} days, {hours} hours, {minutes} minutes and {seconds} seconds"));
            }
        });
    }
}
