pub mod client;
pub mod utils;

use dialoguer::console::style;
use std::fmt::{self, Display};

#[derive(Clone)]
pub struct Branch {
    pub name: String,
    pub selected: bool,
}

impl Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.selected {
            return write!(f, "{} (current)", style(&self.name).underlined().green());
        }

        write!(f, "{}", self.name)
    }
}
