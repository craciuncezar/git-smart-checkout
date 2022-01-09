mod git_branch;
pub use git_branch::GitBranch;
use std::process::{Command, Output};

pub mod utils;

pub fn switch_to_branch(branch: &GitBranch) -> Output {
    Command::new("git")
        .arg("checkout")
        .arg(branch.name.clone())
        .output()
        .expect("Could not run git checkout")
}

pub fn stash_git_changes() -> Output {
    Command::new("git")
        .arg("stash")
        .output()
        .expect("Could not run git stash")
}

pub fn get_git_branches() -> Vec<GitBranch> {
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
