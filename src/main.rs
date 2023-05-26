use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(app)).await?;
    Ok(())
}

async fn app(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["name"].as_str().unwrap_or("Anonymous");

    Ok(json!({ "message": format!("Hey there, {}!", first_name) }))
}
