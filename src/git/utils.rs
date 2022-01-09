use super::GitBranch;

pub fn hoist_selected_branch(branches: Vec<GitBranch>) -> Vec<GitBranch> {
    let mut sorted_branches = branches.to_vec();
    sorted_branches.sort_by(|a, b| b.selected.cmp(&a.selected));
    return sorted_branches;
}
