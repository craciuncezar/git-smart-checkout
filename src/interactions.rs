use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

use crate::git::GitBranch;

pub fn select_branch(branches: &[GitBranch]) -> Result<&GitBranch> {
    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Search git branch by name")
        .default(0)
        .items(&branches[..])
        .interact()?;
    Ok(&branches[selection_index])
}

pub fn confirm_git_stash() -> Result<bool> {
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Stash changes before switch branches?")
        .interact()?)
}
