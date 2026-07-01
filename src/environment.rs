use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Environment {
    pub notion_api_token: String,
    pub notion_root_page: String
}

impl Environment {
    pub fn load() -> Result<Self, String> {
        dotenvy::dotenv().map_err(|e| format!("Missing .env file: {e}"))?;

        let env = envy::from_env().map_err(|e| format!("Error deserializing .env file: {e}"))?;
        Ok(env)
    }
}