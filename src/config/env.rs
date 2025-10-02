use crate::util::deserialize_arc_str;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct Env {
    #[validate(length(min = 1, message = "GEMINI_API_KEY must not be empty"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub gemini_api_key: Arc<str>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        let _ = dotenv();

        let env: Self = envy::from_env().unwrap_or_else(|e| {
            log::error!("{}, exiting ... ", e);
            std::process::exit(1);
        });

        env.validate().unwrap_or_else(|e| {
            let message = e
                .field_errors()
                .values()
                .flat_map(|e| e.iter())
                .filter_map(|err| {
                    err.message
                        .as_ref()
                        .map(|msg| msg.to_string())
                        .or(Some(String::from("invalid value")))
                })
                .next()
                .unwrap_or(String::from("invalid value"));

            log::error!("Environment variable error: {}, exiting ... ", message);
            std::process::exit(1);
        });

        env
    }
}

pub static ENV: Lazy<Env> = Lazy::new(Env::new);
