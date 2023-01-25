extern crate chrono;

mod container;
mod device;
mod io;
mod settings;

use crate::settings::Settings;
use polars::prelude::*;


fn main() {
    let settings = Settings::new();

    let df = df!["A" => &[1, 2, 3]];
}
