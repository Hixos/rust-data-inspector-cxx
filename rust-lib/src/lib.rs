use plotter::PlotterApp;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn run_plotter() -> i32;
    }
}

pub fn run_plotter() -> i32 {
    43
}