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

    pub vercel_settings: VercelSettings,
    pub docker: bool,
}

#[derive(Debug, Default)]
pub struct VercelSettings {
    pub middleware: bool,
}

impl ProjectOptions {
    pub fn prompt() -> Result<Self> {
        let mut options = ProjectOptions::default();

        options.prompt_project_name()?;
        options.prompt_git()?;
        options.prompt_web_server()?;
        options.prompt_websocket_server()?;
        options.prompt_database()?;
        options.prompt_vercel_settings()?;
        options.prompt_docker()?;

        Ok(options)
    }

    fn prompt_project_name(&mut self) -> Result<()> {
        let project_name = inquire::Text::new("Project name: ")
            .with_initial_value("my-awesome-project")
            .prompt()?;

        self.name = project_name;
        self.path = PathBuf::from("./").join(&self.name);
        Ok(())
    }

    fn prompt_git(&mut self) -> Result<()> {
        let init_git = inquire::Confirm::new("Initialize git repository?").prompt()?;
        self.init_git = init_git;
        Ok(())
    }

    fn prompt_web_server(&mut self) -> Result<()> {
        let web_server =
            inquire::Select::new("Select web server:", WebServer::variants()).prompt()?;
        self.web_server = WebServer::from_str(web_server).unwrap();
        Ok(())
    }

    fn prompt_websocket_server(&mut self) -> Result<()> {
        let variants = WebsocketServer::variants(&self);
        if variants.is_empty() {
            return Ok(());
        }

        let websocket_server =
            inquire::Select::new("Select websocket server:", variants).prompt()?;
        self.websocket_server = WebsocketServer::from_str(websocket_server);
        Ok(())
    }

    fn prompt_database(&mut self) -> Result<()> {
        let variants = Database::variants(&self);
        if variants.is_empty() {
            return Ok(());
        }

        let database = inquire::Select::new("Select database:", variants).prompt()?;
        self.database = Database::from_str(database);
        Ok(())
    }

    fn prompt_docker(&mut self) -> Result<()> {
        if self.web_server == WebServer::Cloudflare || self.web_server == WebServer::Vercel {
            return Ok(());
        }

        let use_docker = inquire::Confirm::new("Use docker?").prompt()?;
        self.docker = use_docker;
        Ok(())
    }

    fn prompt_vercel_settings(&mut self) -> Result<()> {
        if self.web_server != WebServer::Vercel {
            return Ok(());
        }

        let middleware =
            inquire::Confirm::new("Use middleware instead of api routes with rewrite?").prompt()?;
        self.vercel_settings.middleware = middleware;

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum WebServer {
    #[default]
    Actix,
    Axum,
    Cloudflare,
    Vercel,
}

impl<'a> WebServer {
    pub fn variants() -> Vec<&'a str> {
        vec!["Actix", "Axum", "Cloudflare (Workers)", "Vercel"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Actix" => Some(WebServer::Actix),
            "Axum" => Some(WebServer::Axum),
            "Cloudflare (Workers)" => Some(WebServer::Cloudflare),
            "Vercel" => Some(WebServer::Vercel),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'a str {
        match self {
            WebServer::Actix => "Actix",
            WebServer::Axum => "Axum",
            WebServer::Cloudflare => "Cloudflare",
            WebServer::Vercel => "Vercel",
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum WebsocketServer {
    #[default]
    Actix,
    Tungstenite,
    On,
    Off,
}

impl<'a> WebsocketServer {
    pub fn variants(options: &ProjectOptions) -> Vec<&'a str> {
        match options.web_server {
            WebServer::Actix => vec!["Tungstenite", "Off"],
            WebServer::Axum => vec!["Tungstenite", "Off"],
            WebServer::Cloudflare => vec!["On", "Off"],
            WebServer::Vercel => vec![],
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Actix" => Some(WebsocketServer::Actix),
            "Tungstenite" => Some(WebsocketServer::Tungstenite),
            "On" => Some(WebsocketServer::On),
            "Off" => Some(WebsocketServer::Off),
            _ => None,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Database {
    #[default]
    Postgres,
    Mysql,
    Planetscale,
    Off,
}

impl<'a> Database {
    pub fn variants(options: &ProjectOptions) -> Vec<&'a str> {
        match options.web_server {
            WebServer::Actix => vec!["Postgres(SQLX)", "Mysql(SQLX)", "Off"],
            WebServer::Axum => vec!["Postgres(SQLX)", "Mysql(SQLX)", "Off"],
            WebServer::Cloudflare => vec!["Planetscale", "Off"],
            WebServer::Vercel => vec!["Planetscale", "Off"],
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Postgres(SQLX)" => Some(Database::Postgres),
            "Mysql(SQLX)" => Some(Database::Mysql),
            "Planetscale" => Some(Database::Planetscale),
            "Off" => Some(Database::Off),
            _ => None,
        }
    }
}

impl<'a> VercelSettings {
    pub fn check_statement(&self, key: &str, value: &str) -> bool {
        match key {
            "MIDDLEWARE" => self.middleware && value == "true",
            _ => false,
        }
    }
}
