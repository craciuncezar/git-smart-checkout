use std::fmt::{self, Display};

use colored::Colorize;

pub struct GitBranch {
    pub name: String,
    pub selected: bool,
}

impl Display for GitBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.selected {
            return write!(f, "{}", self.name.underline().green());
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
