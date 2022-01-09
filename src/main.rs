use git_client::utils;
mod git_client;
mod interactions;

fn main() {
    let branches = utils::hoist_selected_branch(git_client::get_git_branches());
    let selected_branch = interactions::select_branch(&branches);

    if selected_branch.selected {
        println!("You are already on branch {}", selected_branch.name);
        return;
    }

    let switch_branch = git_client::switch_to_branch(&selected_branch);

    if !switch_branch.stderr.is_empty() {
        let error = String::from_utf8_lossy(&switch_branch.stderr);
        if error.contains("Please commit your changes or stash them before you switch branches.") {
            if interactions::confirm_git_stash() {
                git_client::stash_git_changes();
                git_client::switch_to_branch(&selected_branch);
            }
        };
    }
}
