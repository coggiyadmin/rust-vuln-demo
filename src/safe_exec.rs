//! SAFE / TN fixture — subprocess execution without a shell.
//! User input is passed as a distinct argv element via Command::arg, never
//! concatenated into a shell string. The scanner MUST produce ZERO security
//! findings; any command_injection finding is a FALSE POSITIVE.

use std::process::Command;

/// Run `git log` for a branch name. The branch is a separate argument, so even a
/// value like "; rm -rf /" is treated as a literal ref name, not shell syntax.
pub fn git_log(repo_dir: &str, branch: &str) -> std::io::Result<Vec<u8>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_dir)
        .arg("log")
        .arg("--oneline")
        .arg("--") // option terminator
        .arg(branch)
        .output()?;
    Ok(output.stdout)
}

/// Convert an image with ImageMagick; args are an explicit array, no shell.
pub fn thumbnail(input: &str, output: &str) -> std::io::Result<()> {
    Command::new("convert")
        .args(["-resize", "200x200"])
        .arg(input)
        .arg(output)
        .status()?;
    Ok(())
}
