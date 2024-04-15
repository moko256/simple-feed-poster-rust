use crate::{config::Config, misskey_post::MisskeyPost};

pub async fn app_main(args: &[String], config: &Config) {
    let message = args.get(2).unwrap();
    if !config.dryrun {
        let client = MisskeyPost::new(&config.post_misskey_host, &config.post_misskey_api_token);
        client.post(message).await.unwrap();
    } else {
        println!("{} (Dryrun mode.)", message);
    }
}
