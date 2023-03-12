use std::process::{Command, Output, Stdio};

use itertools::Itertools;
use regex::Regex;

use super::{utils, GitBranch};

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

fn get_git_branches() -> Vec<GitBranch> {
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

fn get_list_of_most_recent_branches() -> Vec<String> {
    let reflog_command = Command::new("git")
        .arg("reflog")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep_checkout_command = Command::new("grep")
        .arg("checkout")
        .stdin(Stdio::from(reflog_command.stdout.unwrap()))
        .output()
        .expect("failed to grep checkout");

    let git_reflog_checkout_stdout = String::from_utf8_lossy(&grep_checkout_command.stdout);

    let re = Regex::new(r"checkout: moving from (.*) to").unwrap();

    re.captures_iter(&git_reflog_checkout_stdout)
        .map(|cap| cap[1].to_string())
        .unique()
        .collect::<Vec<String>>()
}

pub fn get_sorted_git_branches() -> Vec<GitBranch> {
    let branches = get_git_branches();
    let most_recent_branches = get_list_of_most_recent_branches();

    utils::hoist_selected_branch(utils::sort_branches_by_name_list(
        branches,
        most_recent_branches,
    ))
}
