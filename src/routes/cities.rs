use super::*;
use crate::models::city::City;
use wither::Model;
// use wither::{prelude::*, Result};
// use wither::bson::{doc, oid::ObjectId};
// use wither::mongodb::Client;
use async_std::stream::StreamExt;
use handlebars::to_json;
use serde_json::value::Map;
use tide::Request;

pub async fn index(req: Request<State>) -> tide::Result {
    let state = &req.state();
    let db = &state.client.database("test");

    let mut cursor = City::find(db.clone(), None, None).await?;
    let mut docs: Vec<City> = Vec::new();

    while let Some(city) = cursor.next().await {
        println!("{:?}", city);

        docs.push(city?);
    }

    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Cities"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("cities".to_string(), to_json(&docs));
    Ok(hb.render_response_ext("cities/list", &data, "html")?)
}
