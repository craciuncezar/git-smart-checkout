use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

use crate::git::Branch;

pub fn select_branch(branches: &Vec<Branch>) -> &Branch {
    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Search git branch by name")
        .default(0)
        .items(&branches[..])
        .interact()
        .unwrap();

    &branches[selection_index]
}

pub fn confirm_git_stash() -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Stash changes before switch branches?")
        .interact()
        .unwrap()
}
