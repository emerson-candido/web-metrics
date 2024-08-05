use std::env;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub endpoints: Vec<String>,
    pub port: u16,
    pub retry: u64
}

pub struct EnvsSettings {
    pub settings_filepath: String,
}

pub fn get_envs() -> Result<EnvsSettings, Box<dyn Error>>{
    let settings_filepath :String = env::var("SETTINGS_FILEPATH")
        .unwrap_or_else(|_| "settings.yaml".to_string());

    Ok(EnvsSettings {
        settings_filepath,
    })
}

pub fn read_config(filepath: &str) -> Result<Config, Box<dyn Error>> {
    let file :File = File::open(filepath)?;
    let reader :BufReader<File>= BufReader::new(file);
    let config :Config = serde_yaml::from_reader(reader)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_get_envs_with_env_var_set() {
        env::set_var("SETTINGS_FILEPATH", "custom_settings.yaml");
        let result = get_envs().expect("Failed to get envs");
        assert_eq!(result.settings_filepath, "custom_settings.yaml");
        env::remove_var("SETTINGS_FILEPATH");
    }

    #[test]
    fn test_get_envs_without_env_var_set() {
        env::remove_var("SETTINGS_FILEPATH");
        let result = get_envs().expect("Failed to get envs");
        assert_eq!(result.settings_filepath, "settings.yaml");
    }

    #[test]
    fn test_read_config_success() {
        let mut file :NamedTempFile = NamedTempFile::new()
            .expect("Failed to create temp file");

        writeln!(
            file,
            r#"
endpoints:
  - https://example.com
  - https://another-example.com

port: 9898
retry: 10
            "#
        )
            .expect("Failed to write to temp file");

        let config :Config = read_config(
            file.path().to_str().unwrap())
            .expect("Failed to read config");

        let expected_config = Config {
            endpoints: vec![
                "https://example.com".to_string(),
                "https://another-example.com".to_string(),
            ],
            port: 9898,
            retry: 10
        };

        assert_eq!(config, expected_config);
    }

    #[test]
    fn test_read_config_invalid_yaml() {
        let mut file :NamedTempFile = NamedTempFile::new()
            .expect("Failed to create temp file");

        writeln!(file, "invalid_yaml_content").expect("Failed to write to temp file");
        let result :Result<Config, Box<dyn Error>> = read_config(file.path().to_str().unwrap());
        assert!(result.is_err());
    }
}
