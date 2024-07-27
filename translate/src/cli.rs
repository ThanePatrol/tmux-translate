use crate::{
    config::AppConfig,
    translate::{TranslationEngine, TranslationRequest},
};
use anyhow::Result;
use clap::Parser;

const API_ENV_NAME: &str = "TRANSLATE_API_KEY";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// translation method
    #[arg(long)]
    engine: Option<TranslationEngine>,

    /// For Google or Feishu
    #[arg(long)]
    api_key: Option<String>,

    /// The source language
    #[arg(long)]
    from: Option<String>,

    /// The destination language
    #[arg(long)]
    to: Option<String>,

    /// The text to translate
    #[arg(long)]
    text: String,
}

pub fn create_translation_request_from_args_and_config(
    args: Args,
    config: &AppConfig,
) -> Result<TranslationRequest> {
    let from_lang = if let Some(from) = args.from {
        from
    } else {
        config.from_lang.clone()
    };

    let to_lang = if let Some(to) = args.to {
        to
    } else {
        config.to_lang.clone()
    };

    let translation_engine = if let Some(engine) = args.engine {
        engine
    } else {
        config.engine.clone()
    };

    // Try read from args, then env, then config
    let api_key = if let Ok(api_from_env) = std::env::var(API_ENV_NAME) {
        Some(api_from_env)
    } else if let Some(args_api) = args.api_key {
        Some(args_api)
    } else {
        if "" != &config.api_key {
            Some(config.api_key.clone())
        } else if translation_engine == TranslationEngine::NLP {
            None
        } else {
            return Err(anyhow::Error::msg("requested for a translation engine that requires an api key but none were provided"));
        }
    };

    Ok(TranslationRequest::new(
        from_lang,
        to_lang,
        args.text,
        translation_engine,
        api_key,
    ))
}

mod test {
    use super::*;

    #[test]
    fn create_translation_request_from_args_and_config_overrides_config() {
        let config = AppConfig::default();
        let from = Some("de".into());
        let to = Some("fr".into());
        let api = Some("123".into());
        let args = Args {
            engine: Some(TranslationEngine::Feishu),
            from: from.clone(),
            to: to.clone(),
            api_key: api.clone(),
            text: "translate me".into(),
        };

        assert_ne!(config.engine, *args.engine.as_ref().unwrap());
        assert_ne!(config.from_lang, *args.from.as_ref().unwrap());
        assert_ne!(config.to_lang, *args.to.as_ref().clone().unwrap());
        assert_eq!(config.api_key, "");

        let req = create_translation_request_from_args_and_config(args, &config).unwrap();

        assert_eq!(req.api_key, api);
        assert_eq!(req.to, to.unwrap());
        assert_eq!(req.from, from.unwrap());
        // TODO do parsing then check equal
    }

    #[test]
    fn create_translation_request_from_args_env_variable_overrides_args_and_config() {
        let cfg_api = "123".to_string();
        let mut config = AppConfig::default();
        config.api_key = cfg_api.clone();

        let from = Some("de".into());
        let to = Some("fr".into());
        let api = Some("456".into());
        let args = Args {
            engine: Some(TranslationEngine::Google),
            from: from.clone(),
            to: to.clone(),
            api_key: api.clone(),
            text: "text".into(),
        };
        std::env::set_var(API_ENV_NAME, "some-key");

        let req = create_translation_request_from_args_and_config(args, &config).unwrap();
        assert_eq!(req.api_key.unwrap(), "some-key");
    }
}
