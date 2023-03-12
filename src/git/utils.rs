use super::GitBranch;

pub fn hoist_selected_branch(mut branches: Vec<GitBranch>) -> Vec<GitBranch> {
    branches.sort_by(|a, b| b.selected.cmp(&a.selected));
    branches
}

pub fn sort_branches_by_name_list(branches: Vec<GitBranch>, list: Vec<String>) -> Vec<GitBranch> {
    let mut sorted_branches = branches.to_vec();
    sorted_branches.sort_by(|a, b| {
        let a_index = list.iter().position(|x| x == &a.name);
        let b_index = list.iter().position(|x| x == &b.name);

        match (a_index, b_index) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    return sorted_branches;
}
