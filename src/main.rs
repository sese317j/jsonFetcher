use mongodb::Client;

use serde_json;

use std::error::Error;
use bson::{Bson, doc, Document};

use futures::stream::TryStreamExt;

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
        // println!("Uri: {}", uri);

        let uri_trimmed = uri.trim_matches('"');

        let response = reqwest::get(uri_trimmed).await?;

        let response_string = response.text().await?;

        let bson_data_objekt: Bson = serde_json::from_str(&response_string)?;

        let _update_result = metadata_collection.update_one(
            doc! {
            "_id": &item.get("_id")
            },
            doc! {
            "$set": { "data": &bson_data_objekt }
            },
            None,
        ).await?;

        //println!("Updated {} document", update_result.modified_count);
        //println!("{}",v);
    }

    // Needed for async await ? or maybe not ?
    Ok(())
}
