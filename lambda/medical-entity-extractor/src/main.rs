use aws_config::from_env;
use aws_sdk_comprehendmedical::error::SdkError;
use aws_sdk_comprehendmedical::types::Entity;
use aws_sdk_comprehendmedical::Client;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing::error;
use tracing::info;
use tracing_subscriber;
 
#[derive(Deserialize)]
struct Request {
    text: String,
}

#[derive(Serialize)]
struct Response {
    entities: Vec<ExtractedEntity>,
}

#[derive(Serialize)]
struct ExtractedEntity {
    text: String,
    category: String,
    type_: String,
    score: f32,
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
    info!("Rust function invoked with text: {:?}", event.text);

    let config = from_env().load().await;
    let client = Client::new(&config);

    match client.detect_entities_v2().text(event.text).send().await {
        Ok(response) => {
            let entities = response
                .entities
                .into_iter()
                .map(map_entity_to_extracted_entity)
                .collect();
            Ok(Response { entities })
        }
        Err(err) => {
            error!("Error: {:?}", err);
            Err(format!("Unexpected error: {}", err).into())
        }
    }
}

fn map_entity_to_extracted_entity(entity: Entity) -> ExtractedEntity {
    ExtractedEntity {
        text: entity.text.unwrap_or_default(),
        category: entity
            .category
            .map_or("Unknown".to_string(), |c| format!("{:?}", c)),
        type_: entity
            .r#type
            .map_or("Unknown".to_string(), |t| format!("{:?}", t)),
        score: entity.score.unwrap_or(0.0),
    }
}
