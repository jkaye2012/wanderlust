use std::env;

use google_signin::Client;

pub fn create_google_client() -> Client {
    let client_id = env::var("GOOGLE_OAUTH_CLIENT_ID")
        .expect("Google client id environment variable was not set");
    let mut client = Client::new();
    client.audiences.push(client_id);
    client
}
