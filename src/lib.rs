#![warn(warnings)]

pub mod model;

mod client;
mod errors;
mod session;

pub use client::Client;
pub use errors::*;

use session::Session;

static COASTAL_DEPARTMENT_LIST: &[&str] = &[
    "06", "11", "13", "14", "17", "22", "29", "2A", "2B", "30", "33", "34", "35", "40", "44", "50",
    "56", "59", "62", "64", "66", "76", "80", "83", "85",
];
