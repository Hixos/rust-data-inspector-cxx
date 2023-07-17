#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use plotter::{SignalGroup, SignalSample};
use std::sync::mpsc::Sender;
use std::time::Instant;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type CxxSignalGroup;
        type CxxSignal;

        fn run_gui(signals: Box<CxxSignalGroup>);

        fn new_signal_group() -> Box<CxxSignalGroup>;       
        fn add_signal(self: &mut CxxSignalGroup, name: &str) -> Box<CxxSignal>;

        fn push_sample(self: &CxxSignal, value: f64);
    }
}

pub struct CxxSignalGroup {
    start_time: Instant,
    signals: plotter::SignalGroup,
}

pub struct CxxSignal {
    start_time: Instant,
    sender: Sender<SignalSample>,
}

pub fn new_signal_group() -> Box<CxxSignalGroup> {
    Box::new(CxxSignalGroup {
        start_time: Instant::now(),
        signals: SignalGroup::new(),
    })
}

impl CxxSignalGroup {
    pub fn add_signal(&mut self, name: &str) -> Box<CxxSignal> {
        let sender = self.signals.add_signal(name);

        Box::new(CxxSignal {
            start_time: self.start_time,
            sender,
        })
    }
}

pub fn run_gui(signals: Box<CxxSignalGroup>) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Plotter",
        native_options,
        Box::new(|cc| Box::new(plotter::PlotterApp::run(cc, signals.signals))),
    )
    .expect("Error running eframe");
}

impl CxxSignal {
    pub fn push_sample(&self, value: f64) {
        let t = Instant::now() - self.start_time;
        self.sender
            .send(SignalSample {
                t: t.as_secs_f64(),
                y: value,
            })
            .expect("Error sending sample");
    }
}
