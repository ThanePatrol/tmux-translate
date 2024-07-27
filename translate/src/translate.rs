use anyhow::Result;
use std::borrow::Borrow;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum, Deserialize, Serialize, PartialEq, Eq)]
pub enum TranslationEngine {
    Google,
    NLP,
    Feishu,
}

impl ToString for TranslationEngine {
    fn to_string(&self) -> String {
        match self {
            TranslationEngine::Google => "google",
            TranslationEngine::NLP => "nlp",
            TranslationEngine::Feishu => "feishu",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct TranslationRequest {
    pub from: String,
    pub to: String,
    pub text: String,
    pub engine: TranslationEngine,
    pub api_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TranslationArgs {
    from: String,
    to: String,
    text: String,
}

impl TranslationRequest {
    pub fn new(
        from: String,
        to: String,
        text: String,
        engine: TranslationEngine,
        api_key: Option<String>,
    ) -> Self {
        TranslationRequest {
            from,
            to,
            text,
            engine,
            api_key,
        }
    }

    // TODO, maybe take ownership, rather than cloning
    pub fn get_request_json(&self) -> String {
        let args = TranslationArgs {
            from: self.from.clone(),
            to: self.to.clone(),
            text: self.text.clone(),
        };

        serde_json::to_string(&args).unwrap()
    }
}

// TODO good timeout values
pub fn translate_text(req: TranslationRequest) -> Result<String> {
    let api_key = req
        .api_key
        .borrow()
        .as_ref()
        .expect("api key has not been set");

    let client = reqwest::blocking::Client::new();
    let rsp = client
        .post("https://google-translate113.p.rapidapi.com/api/v1/translator/text")
        .header("Content-Type", "application/json")
        .header("x-rapidapi-host", "google-translate113.p.rapidapi.com")
        .header("x-rapidapi-key", api_key)
        .body(req.get_request_json())
        .send();

    let body = rsp?.text()?;
    Ok(body)
}
