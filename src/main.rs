mod git;
mod interactions;

fn main() {
    ctrlc::set_handler(show_cursor).unwrap();

    let branches = git::client::get_sorted_git_branches();
    let selected_branch = interactions::select_branch(&branches).unwrap_or_else(|_| {
        show_cursor();
        std::process::exit(0);
    });

    if selected_branch.selected {
        println!("You are already on branch {}", selected_branch.name);
        return;
    }

    let switch_branch = git::client::switch_to_branch(selected_branch);

    if !switch_branch.stderr.is_empty() {
        let error = String::from_utf8_lossy(&switch_branch.stderr);
        if error.contains("Please commit your changes or stash them before you switch branches.") {
            let should_stash = interactions::confirm_git_stash().unwrap_or_else(|_| {
                show_cursor();
                std::process::exit(0);
            });
            if should_stash {
                git::client::stash_git_changes();
                git::client::switch_to_branch(selected_branch);
            }
        };
    }
}

// make sure that the cursor re-appears when interrupting
fn show_cursor() {
    let term = dialoguer::console::Term::stderr();
    let _ = term.show_cursor();
}
