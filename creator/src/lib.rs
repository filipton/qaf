use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct ProjectOptions {
    pub name: String,
    pub path: PathBuf,
    pub init_git: bool,
    pub web_server: WebServer,
    pub websocket_server: Option<WebsocketServer>,
    pub database: Option<Database>,
}

impl ProjectOptions {
    pub fn prompt() -> Result<Self> {
        let mut options = ProjectOptions::default();

        let project_name = inquire::Text::new("Project name: ")
            .with_initial_value("my-awesome-project")
            .prompt()?;
        options.name = project_name;
        options.path = PathBuf::from("./").join(&options.name);

        let init_git = inquire::Confirm::new("Initialize git repository?").prompt()?;
        options.init_git = init_git;

        let web_server =
            inquire::Select::new("Select web server:", WebServer::variants()).prompt()?;
        options.web_server = WebServer::from_str(web_server).unwrap();

        let websocket_server = inquire::Select::new(
            "Select websocket server:",
            WebsocketServer::variants(&options.web_server),
        )
        .prompt()?;
        options.websocket_server = WebsocketServer::from_str(websocket_server);

        let database = inquire::Select::new("Select database:", Database::variants()).prompt()?;
        options.database = Database::from_str(database);

        Ok(options)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum WebServer {
    #[default]
    Actix,
    Axum,
}

impl<'a> WebServer {
    pub fn variants() -> Vec<&'a str> {
        //vec!["Actix", "Axum"]
        vec!["Actix"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Actix" => Some(WebServer::Actix),
            "Axum" => Some(WebServer::Axum),
            _ => None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum WebsocketServer {
    #[default]
    Actix,
    Tungstenite,
    Off,
}

impl<'a> WebsocketServer {
    pub fn variants(web_server: &WebServer) -> Vec<&'a str> {
        vec!["Tungstenite", "Off"]
        /*
        match web_server {
            WebServer::Actix => vec!["Actix", "Tungstenite", "Off"],
            WebServer::Axum => vec!["Tungstenite", "Off"],
        }
        */
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Actix" => Some(WebsocketServer::Actix),
            "Tungstenite" => Some(WebsocketServer::Tungstenite),
            _ => None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Database {
    #[default]
    Postgres,
    Off,
}

impl<'a> Database {
    pub fn variants() -> Vec<&'a str> {
        vec!["Postgres(SQLX)", "Off"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Postgres(SQLX)" => Some(Database::Postgres),
            _ => None,
        }
    }
}
