use dialoguer::console::style;
use std::fmt::{self, Display};

pub struct GitBranch {
    pub name: String,
    pub selected: bool,
}

impl Display for GitBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.selected {
            return write!(f, "{} (current)", style(&self.name).underlined().green());
        }

        write!(f, "{}", self.name)
    }
}

impl Clone for GitBranch {
    fn clone(&self) -> Self {
        GitBranch {
            name: self.name.clone(),
            selected: self.selected,
        }
    }
}
