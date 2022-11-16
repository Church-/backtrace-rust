#[macro_use]
extern crate serde_json;

pub mod error;
pub mod panic;
mod sender;

use std::collections::HashMap;

pub use error::init;
pub use error::ResultExt;
pub use panic::register_error_handler;

#[derive(Debug, Clone)]
pub struct SubmissionTarget {
    token: String,
    url: String,
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub annotations: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
}
