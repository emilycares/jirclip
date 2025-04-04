use std::io::ErrorKind;

use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Configuration {
    pub instances: Vec<JiraInstance>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct JiraInstance {
    pub prefix: String,
    pub organisations: Vec<String>,
}

pub async fn location() -> Option<String> {
    if let Some(mut dir) = dirs::config_dir() {
        dir.push("jirclip");
        if let Some(dir) = dir.to_str() {
            if fs::create_dir_all(dir).await.is_err() {
                return None;
            }
        }
        dir.push("jirclip");
        dir.set_extension("json");

        if let Some(file) = dir.to_str() {
            return Some(file.to_string());
        }
    }

    None
}

pub async fn load_configuration(location: &str) -> Configuration {
    println!("Load Configuration: {location}");
    match fs::read_to_string(&location).await {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|error| {
            println!("Could not load configuration file: {error:?}");
            Configuration::default()
        }),
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                save_configuration(
                    Configuration {
                        instances: vec![JiraInstance {
                            organisations: vec![
                                "COOL_PROJECT".to_string(),
                                "NICE_PROJECT".to_string(),
                            ],
                            prefix: "https://jira.copany.com/browse/{TICKET}".to_string(),
                        }],
                    },
                    location,
                )
                .await
                .expect("Unable to write config");
            }

            Configuration::default()
        }
    }
}

/// Save configuration to filesystem
async fn save_configuration(
    configuration: Configuration,
    location: &str,
) -> Result<(), std::io::Error> {
    let config: String = serde_json::to_string_pretty(&configuration)?;
    let mut file = File::create(location).await?;

    file.write_all(config.as_bytes()).await?;

    file.flush().await?;

    Ok(())
}
