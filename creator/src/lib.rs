use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ProjectOptions {
    pub name: String,
    pub path: PathBuf,
    pub init_git: bool,
    pub generate_readme: bool,
    pub web_server: WebServer,
    pub websocket_server: Option<WebsocketServer>,
    pub database: Option<Database>,
}

impl ProjectOptions {
    pub fn prompt() -> Self {
        let mut options = ProjectOptions::default();

        let project_name = inquire::Text::new("Project name: ")
            .with_initial_value("my-awesome-project")
            .prompt();
        options.name = project_name.unwrap();
        options.path = PathBuf::from("./").join(&options.name);

        let init_git = inquire::Confirm::new("Initialize git repository?").prompt();
        options.init_git = init_git.unwrap();

        if options.init_git {
            let generate_readme = inquire::Confirm::new("Generate README.md?").prompt();
            options.generate_readme = generate_readme.unwrap();
        }

        let web_server = inquire::Select::new("Select web server:", WebServer::variants()).prompt();
        options.web_server = WebServer::from_str(web_server.unwrap()).unwrap();

        let websocket_server =
            inquire::Select::new("Select websocket server:", WebsocketServer::variants()).prompt();
        options.websocket_server = WebsocketServer::from_str(websocket_server.unwrap());

        let database = inquire::Select::new("Select database:", Database::variants()).prompt();
        options.database = Database::from_str(database.unwrap());

        return options;
    }
}

#[derive(Debug, Default)]
pub enum WebServer {
    #[default]
    Actix,
    Axum,
}

impl<'a> WebServer {
    pub fn variants() -> Vec<&'a str> {
        vec!["Actix", "Axum"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Actix" => Some(WebServer::Actix),
            "Axum" => Some(WebServer::Axum),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub enum WebsocketServer {
    #[default]
    Actix,
    Tungstenite,
    Off,
}

impl<'a> WebsocketServer {
    pub fn variants() -> Vec<&'a str> {
        vec!["Actix", "Tungstenite", "Off"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Actix" => Some(WebsocketServer::Actix),
            "Tungstenite" => Some(WebsocketServer::Tungstenite),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub enum Database {
    #[default]
    Postgres,
    Off,
}

impl<'a> Database {
    pub fn variants() -> Vec<&'a str> {
        vec!["Postgres", "Off"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Postgres" => Some(Database::Postgres),
            _ => None,
        }
    }
}