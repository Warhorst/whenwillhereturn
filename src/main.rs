use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::{Local, TimeDelta, TimeZone, Utc};
use eframe::{App, Frame};
use eframe::epaint::{FontFamily, FontId};
use egui::{CentralPanel, Context, TextStyle};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let delta_until_return = Arc::new(Mutex::new(TimeDelta::default()));

    let loop_delta_until_return = Arc::clone(&delta_until_return);
    thread::spawn(move || {
        let return_date = Local.with_ymd_and_hms(2024, 10, 1, 9, 30, 0).unwrap();

        loop {
            thread::sleep(Duration::from_secs(1));

            let now = Utc::now();
            let diff = return_date.signed_duration_since(now);

            let mut delta_until_return = loop_delta_until_return.lock().unwrap();
            *delta_until_return = diff;
        }
    });

    eframe::run_native(
        "When will he return?",
        eframe::NativeOptions::default(),
        Box::new(|_| {
            Box::<WhenWillHeReturnApp>::new(WhenWillHeReturnApp {
                delta_until_return
            })
        })
    )
}

struct WhenWillHeReturnApp {
    delta_until_return: Arc<Mutex<TimeDelta>>
}

impl App for WhenWillHeReturnApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles.insert(
                TextStyle::Heading,
                FontId::new(36.0, FontFamily::Proportional),
            );

            // Yeah, interactive. I am definitely not too lazy to rewrite this in a UI with continuous updates.
            ui.heading("When will he return? This is an interactive counter, so move your mouse to find out!");

            let delta = self.delta_until_return.lock().unwrap();

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
