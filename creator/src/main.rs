use creator::ProjectOptions;
use which::which;

fn main() {
    let git_path = which("git").expect("Git is not installed!");
    let options = ProjectOptions::prompt();

    println!("{:?}, {:?}", options, git_path);
}
