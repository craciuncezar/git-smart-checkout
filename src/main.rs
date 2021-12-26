use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use std::{
    fmt,
    fmt::Display,
    process::{Command, Output},
};

struct GitBranch {
    name: String,
    selected: bool,
}

impl Display for GitBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.selected {
            return write!(f, "{}", self.name.underline().green());
        }

        write!(f, "{}", self.name)
    }
}

fn main() {
    let branches = get_git_branches();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Fuzzy search git branches")
        .default(0)
        .items(&branches[..])
        .interact()
        .unwrap();

    let switch_branch = switch_to_branch(&branches[selection]);

    if !switch_branch.stderr.is_empty() {
        let error = String::from_utf8_lossy(&switch_branch.stderr);
        error
            .contains("Please commit your changes or stash them before you switch branches.")
            .then(|| {
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Stash changes before switch branches?")
                    .interact()
                    .unwrap()
                {
                    stash_git_changes();
                    switch_to_branch(&branches[selection]);
                }
            });
    }
}

fn get_git_branches() -> Vec<GitBranch> {
    let git_branch_command = Command::new("git")
        .arg("branch")
        .output()
        .expect("failed to get list of git branches");

    let git_branch_stdout = String::from_utf8_lossy(&git_branch_command.stdout);

    git_branch_stdout
        .lines()
        .map(|branch| {
            if branch.starts_with("*") {
                return GitBranch {
                    selected: true,
                    name: String::from(branch[1..].trim()),
                };
            }

            GitBranch {
                selected: false,
                name: String::from(branch.trim()),
            }
        })
        .collect()
}

fn switch_to_branch(branch: &GitBranch) -> Output {
    Command::new("git")
        .arg("checkout")
        .arg(branch.name.clone())
        .output()
        .expect("Could not run git checkout")
}

fn stash_git_changes() -> Output {
    Command::new("git")
        .arg("stash")
        .output()
        .expect("Could not run git stash")
}
