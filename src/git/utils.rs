use super::GitBranch;

pub fn hoist_selected_branch(mut branches: Vec<GitBranch>) -> Vec<GitBranch> {
    branches.sort_by(|a, b| b.selected.cmp(&a.selected));
    branches
}
