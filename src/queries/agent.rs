use crate::helpers::Credentials;
use crate::spacetraders_api::responses::Agent;
use reqwest::Client;

pub async fn agent(client: &Client, credentials: &Credentials) -> Result<Agent, reqwest::Error> {
    client
        .get("https://api.spacetraders.io/v2/my/agent")
        .bearer_auth(&credentials.token)
        .send()
        .await?
        .json()
        .await
}
