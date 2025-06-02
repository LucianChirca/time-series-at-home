pub mod server;
pub mod endpoints;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub scheme: String,
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn new(scheme: String, host: String, port: u16) -> Self {
        Self { scheme, host, port }
    }

    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }
}