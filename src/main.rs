use fun_plots::fun_plots::{mand, wisteria, perk_sq_new};
use poloto::build::{markers, text};
use poloto::prelude::*;
use std::fs::File;
use std::io::Write;
mod fun_plots;
macro_rules! scatter {
    ($title:expr, $x:expr, $y:expr, $($function:expr);*, $data: expr) => {
        quick_fmt!(
            $title,
            $x,
            $y,
            $($function,)*
            $data.into_iter().cloned_plot().scatter(""))
    };
}
macro_rules! write_to_file {
    ($filepath:expr, $text:expr) => {
        write!(File::create($filepath)?, "{}", $text)
    };
}
fn main() -> std::io::Result<()> {
    //let my_data = (-100..100).map(|i| (i as f64, ((i as i32).pow(3)) as f64));
    //let my_data = mand(1000, 3.0, 3.0, -1.5, -1.5);
    //let my_data = wisteria(500);
    let my_data = perk_sq_new();
    let plotter =
        scatter!("Perk", "x", "y", , my_data);
    write_to_file!(
        "/home/kartonrealista/Pictures/perk.svg",
        poloto::disp(|w| plotter.simple_theme_dark(w))
    )
}
