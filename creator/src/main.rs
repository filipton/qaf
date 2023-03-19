use inquire::Text;

#[derive(Debug, Default)]
struct ProjectOptions {
    name: String,
    init_git: bool,
}

fn main() {
    let mut options = ProjectOptions::default();

    let project_name = Text::new("Project name: ")
        .with_initial_value("my-awesome-project")
        .prompt();
    options.name = project_name.unwrap();

    let init_git = inquire::Confirm::new("Initialize git repository?").prompt();
    options.init_git = init_git.unwrap();

    println!("{:?}", options);
}
