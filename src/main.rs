use std::process::Command;

fn main() {
    let local_branches = Command::new("git")
        .arg("branch")
        .output()
        .expect("failed to get list of git branches");

    println!(
        "stdout: {}",
        String::from_utf8_lossy(&local_branches.stdout)
    );
}
