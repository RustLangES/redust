use toml::Value;

pub struct RedustConfig {
    pub admin_password: String,
    pub address: String,
}

impl RedustConfig {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = std::fs::read_to_string(path)?;
        let config: Value = config.parse()?;

        let admin_password = config
            .get("password")
            .and_then(|value| value.as_str())
            .unwrap_or("password")
            .to_owned();

        let address = config
            .get("address")
            .and_then(|value| value.as_str())
            .unwrap_or("localhost:6969")
            .to_owned();

        Ok(Self {
            admin_password: admin_password.to_string(),
            address: address.to_string(),
        })
    }
}
