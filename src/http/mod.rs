pub mod server;
pub mod endpoints;
pub mod response;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}