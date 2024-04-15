use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct MisskeyPost<'a> {
    client: Client,
    host: &'a str,
    token: &'a str,
}

impl MisskeyPost<'_> {
    pub fn new<'a>(host: &'a str, token: &'a str) -> MisskeyPost<'a> {
        let client = Client::builder()
            .user_agent(env!("CARGO_PKG_NAME"))
            .pool_max_idle_per_host(0)
            .tcp_keepalive(None)
            .build()
            .unwrap();

        MisskeyPost {
            client,
            host,
            token,
        }
    }

    pub async fn post(&self, text: &str) -> Result<(), reqwest::Error> {
        let param = NotesCreateParam {
            i: self.token,
            text,
        };
        let _request = self
            .client
            .post(format!("https://{}/api/notes/create", self.host))
            .json(&param)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct NotesCreateParam<'a> {
    i: &'a str,
    text: &'a str,
}
