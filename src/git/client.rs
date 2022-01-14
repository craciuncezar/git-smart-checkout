use std::process::{Command, Output};

use super::GitBranch;

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
            if let Some(stripped) = branch.strip_prefix('*') {
                return GitBranch {
                    selected: true,
                    name: String::from(stripped.trim()),
                };
            }

            GitBranch {
                selected: false,
                name: String::from(branch.trim()),
            }
        })
        .collect()
}
