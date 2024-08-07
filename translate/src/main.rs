use std::process::exit;

use clap::Parser;
use cli::create_translation_request_from_args_and_config;
use cli::Args;
use config::load_config_file_or_default;
use translate::translate_text;
use translate::TranslationEngine;

mod cli;
mod config;
mod translate;

fn main() {
    let args = Args::parse();

    // Rely on HOME env var on POSIX systems
    let config_result = load_config_file_or_default("HOME".into());

    let config = match config_result {
        Ok(conf) => conf,
        Err(err) => {
            println!("couldn't load config file! error is {err}");
            exit(1);
        }
    };

    let translation_request_result = create_translation_request_from_args_and_config(args, &config);

    let translation_request = match translation_request_result {
        Ok(req) => req,
        Err(err) => {
            println!("couldn't build translation request. error is {err}");
            exit(1);
        }
    };

    let translated_result = translate_text(translation_request);
    let translated = match translated_result {
        Ok(trans) => trans,
        Err(err) => {
            println!("couldn't make call to api. error is {err}");
            exit(1);
        }
    };
    println!("{}", translated);
}
