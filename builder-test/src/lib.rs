#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::mysql::MySqlPool,
}

pub struct StartupOptions {
    pub connection_string: String,
    pub max_connections: u32,

    pub bind_address: String,
}

impl StartupOptions {
    pub fn new() -> Self {
        Self {
            connection_string: std::env::var("DATABASE_URL")
                .expect("You must set DATABASE_URL env variable!"),

            max_connections: std::env::var("MAX_CONNECTIONS")
                .unwrap_or("5".to_string())
                .parse()
                .unwrap(),

            bind_address: std::env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8080".to_string()),
        }
    }
}
