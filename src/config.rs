use std::{env, error::Error};

use dotenvy::dotenv;

pub struct Config {
    pub message_format: String,
    pub post_url: String,
    pub post_body_type: String,
    pub post_body: String,
}

pub fn load_config() -> Config {
    dotenv().unwrap();

    Config {
        message_format: env::var("MESSAGE_FORMAT").unwrap(),
        post_url: env::var("POST_URL").unwrap(),
        post_body_type: env::var("POST_BODY_TYPE").unwrap(),
        post_body: env::var("POST_BODY").unwrap(),
    }
}
