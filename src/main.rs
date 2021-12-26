use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use git_client::{utils, GitClient};
mod git_client;

fn main() {
    let branches = utils::hoist_selected_branch(GitClient::get_git_branches());

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Fuzzy search git branches")
        .default(0)
        .items(&branches[..])
        .interact()
        .unwrap();

    let switch_branch = GitClient::switch_to_branch(&branches[selection]);

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
                    GitClient::stash_git_changes();
                    GitClient::switch_to_branch(&branches[selection]);
                }
            });
    }
}
