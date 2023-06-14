use reqwest::RequestBuilder;
use reqwest::Response;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct Query {
    pub request: RequestBuilder,
    pub response: Sender<Result<Response, Box<dyn std::error::Error + Send + Sync>>>,
}
