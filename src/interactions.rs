use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

use crate::git_client::GitBranch;

pub fn select_branch(branches: &Vec<GitBranch>) -> &GitBranch {
    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Fuzzy search git branches")
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
