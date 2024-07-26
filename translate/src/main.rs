use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct TranslationRequest {
    from: String,
    to: String,
    text: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// translation method
    #[arg(short, long, default_value_t = TranslationEngine::Google)]
    name: TranslationEngine,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::new())]
    api_key: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum TranslationEngine {
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

fn main() {
    let args = Args::parse();
    let t = TranslationRequest {
        text: String::from("hello how are you?"),
        from: String::from("en"),
        to: String::from("zh-cn"),
    };

    println!("{:?}", args);

    let translated = translate_text_with_google(t, args.api_key);
    println!("{}", translated);
}

// TODO good timeout values
fn translate_text_with_google(req: TranslationRequest, api_key: String) -> String {
    let req_json = serde_json::to_string(&req).unwrap();
    let client = reqwest::blocking::Client::new();
    let rsp = client
        .post("https://google-translate113.p.rapidapi.com/api/v1/translator/text")
        .header("Content-Type", "application/json")
        .header("x-rapidapi-host", "google-translate113.p.rapidapi.com")
        .header("x-rapidapi-key", api_key)
        .body(req_json)
        .send();

    if let Ok(success_response) = rsp {
        let body = success_response.text();
        if let Ok(success_body) = body {
            success_body
        } else {
            body.unwrap_err().to_string()
        }
    } else {
        rsp.unwrap_err().to_string()
    }
}
