use std::io::Read;

use anyhow::anyhow;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    pub auth: (
        Option<cliue_core::oauth::TokenRequestor>,
        Option<cliue_core::oauth::TokenData>,
    ),
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        match directories::ProjectDirs::from("com", "DM Earth", "Cliue") {
            Some(dirs) => toml::from_str(&{
                let mut string = String::new();
                match std::fs::File::open(dirs.config_dir().join("cliue.toml")) {
                    Ok(mut file) => file.read_to_string(&mut string)?,
                    Err(_) => {
                        return Ok(Self::default());
                    }
                };
                string
            })
            .map_err(|err| anyhow!(err)),
            None => Err(anyhow!("Invalid project dir!")),
        }
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let dir = (match directories::ProjectDirs::from("com", "DM Earth", "Cliue") {
            Some(dir) => Ok(dir),
            None => Err(anyhow!("Invalid project dir!")),
        })?
        .config_dir()
        .join("cliue.toml");
        tokio::fs::write(dir, toml::to_string(self)?).await?;
        Ok(())
    }
}
