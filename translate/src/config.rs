use crate::TranslationEngine;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub engine: TranslationEngine,
    pub from_lang: String,
    pub to_lang: String,
    pub api_key: String,
}

impl AppConfig {
    pub fn default() -> Self {
        AppConfig {
            engine: TranslationEngine::Google,
            from_lang: String::from("en"),
            to_lang: String::from("zh-cn"),
            api_key: String::from(""),
        }
    }
}

// Only support ~/.config/translate/translate.toml atm
pub fn load_config_file_or_default(home_env_var: String) -> Result<AppConfig> {
    let home_dir = std::env::var(home_env_var).expect("HOME environment variable not set");

    let file_paths = vec![format!("{home_dir}/.config/translate/translate.toml")];

    for path in file_paths.iter() {
        if let Ok(file_as_str) = std::fs::read_to_string(path) {
            let app_config = toml::from_str(&file_as_str)?;
            return Ok(app_config);
        }
    }

    let default_config = AppConfig::default();
    let cfg_string = toml::to_string_pretty(&default_config)?;

    let dir = format!("{home_dir}/.config/translate");

    std::fs::create_dir_all(&dir)?;

    let file = format!("{dir}/translate.toml");

    std::fs::write(file, cfg_string)?;

    Ok(default_config)
}

mod test {
    use super::*;

    const ENV_NAME: &str = "DUMMY_HOME";
    const TEST_FILE: &str = "./testing/.config/translate/translate.toml";
    const TEST_DIR: &str = "./testing/.config/translate";

    #[test]
    fn test_config_creates_default_dir() {
        setup_env();
        let cfg = load_config_file_or_default(ENV_NAME.to_string());

        let conf: AppConfig =
            toml::from_str(std::fs::read_to_string(TEST_FILE).unwrap().as_str()).unwrap();

        let cfg = cfg.unwrap();
        cleanup_dir();
        assert_eq!(conf.from_lang, cfg.from_lang);
        assert_eq!(conf.to_lang, cfg.to_lang);
        assert_eq!(conf.engine, cfg.engine);
    }

    fn cleanup_dir() {
        std::fs::remove_dir_all("./testing").unwrap();
    }

    fn setup_env() {
        std::env::set_var(ENV_NAME, "./testing");
    }

    #[test]
    fn test_load_config_file_or_default_does_not_delete_current_cfg() {
        setup_env();
        std::fs::create_dir_all(TEST_DIR).unwrap();
        let config = AppConfig {
            engine: TranslationEngine::Google,
            from_lang: "dummy".into(),
            to_lang: "memes".into(),
            api_key: "123".into(),
        };
        let as_str = toml::to_string_pretty(&config).unwrap();
        std::fs::write(TEST_FILE, as_str).unwrap();

        let parsed_cfg = load_config_file_or_default(ENV_NAME.to_string());
        let parsed_cfg = parsed_cfg.unwrap();
        cleanup_dir();
        assert_eq!(config.from_lang, parsed_cfg.from_lang);
        assert_eq!(config.to_lang, parsed_cfg.to_lang);
        assert_eq!(config.engine, parsed_cfg.engine);
        assert_eq!(config.api_key, parsed_cfg.api_key);
    }
}
