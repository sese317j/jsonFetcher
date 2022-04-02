use mongodb::Client;
use mongodb::bson::doc;

use serde::Deserialize;


use serde_json::{Value};

use std::error::Error;
use bson::Document;

use futures::stream::TryStreamExt;
use futures::TryFutureExt;

use tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {

    // MongoDB Connection String
    let client_uri = "mongodb://localhost:27017";

    // create mongodb Client with the connection String
    let client = Client::with_uri_str(&client_uri).await?;

    // Get one collection of one Database on the mongoDB cluster
    let metadata_collection = client.database("SolanaTest").collection::<Document>("MetadataCollection");

    let mut cursor = metadata_collection.find(None,None).await?;

    while let Some(item) = cursor.try_next().await? {

        let uri = item.get("uri").unwrap().to_string();
        println!("Uri: {}", uri);

        let uri_trimmed = uri.trim_matches('"');

        let response = reqwest::get(uri_trimmed).await?;

        let data = response.text().await?;

        let v: Value = serde_json::from_str(&data)?;
        println!("{}",v);
    }

    // Needet for async await ? or maybe not ?
    Ok(())
}
