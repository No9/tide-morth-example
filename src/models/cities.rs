use async_std::stream::StreamExt;
use mongodb::{bson::Document, options::ClientOptions, Client, bson};
use serde::{ Serialize, Deserialize };
use std::error;

#[derive(Serialize, Deserialize)]
pub struct City {
    #[serde(rename = "_id")]
    pub mid: bson::oid::ObjectId,
    pub name: String,
    pub country: String
}

pub async fn get() -> Result<Vec<City>, Box<dyn error::Error>> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    let db = client.database("test");
    let coll = db.collection("cities");
    let mut cursor = coll.find(None, None).await?;
    let mut docs: Vec<City> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                // From to BSON isn't the best experience and an issue has been raised
                // https://jira.mongodb.org/projects/RUST/issues/RUST-503
                let city: City = bson::from_bson(bson::to_bson(&document)?).unwrap();
                docs.push(city);
                //.append(document);
            }
            Err(e) => println!("Err {}", e),
        }
    }

    Ok(docs)
}
