use aws_sdk_comprehend::Client;
use aws_sdk_comprehend::types::LanguageCode;
use aws_config::from_env;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use tracing::error;
use tracing_subscriber;

#[derive(Deserialize)]
struct Request {
    text: String,
}

#[derive(Serialize)]
struct Response {
    sentiment: String,
    sentiment_score: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("Logger initialized");

    let func = service_fn(handle_request);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handle_request(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (event, _context) = event.into_parts();
    info!("Received event: {:?}", event.text);

    let config = from_env().load().await;
    let client = aws_sdk_comprehend::Client::new(&config);

    match client
        .detect_sentiment()
        .language_code(LanguageCode::En)
        .text(event.text)
        .send()
        .await
    {
        Ok(sentiment_result) => {
            let sentiment = sentiment_result.sentiment().map_or("UNKNOWN".to_string(), |s| s.as_str().to_string());

            let sentiment_score = if let Some(scores) = sentiment_result.sentiment_score() {
                Some(json!({
                    "Positive": scores.positive(),
                    "Negative": scores.negative(),
                    "Neutral": scores.neutral(),
                    "Mixed": scores.mixed()
                }))
            } else {
                None
            };

            info!("Sentiment detected: {}", sentiment);
            info!("Sentiment scores: {:?}", sentiment_score);

            Ok(Response {
                sentiment,
                sentiment_score,
            })
        }
        Err(err) => {
            error!("Failed to detect sentiment: {:?}", err);
            Err(err.into())
        }
    }
}
