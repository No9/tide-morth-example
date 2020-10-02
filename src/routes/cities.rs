use super::*;
use crate::models::city::City;
use async_std::stream::StreamExt;
use handlebars::to_json;
use serde_json::value::Map;
use tide::Request;
use tracing::{info, instrument};
use wither::bson::{doc, oid::ObjectId};
use wither::Model;

#[instrument]
pub async fn index(req: Request<State>) -> tide::Result {
    info!("Tracing Started");
    let state = &req.state();
    let db = &state.client.database("test");

    let mut cursor = City::find(&db.clone(), None, None).await?;
    let mut docs: Vec<City> = Vec::new();

    while let Some(city) = cursor.next().await {
        docs.push(city?);
    }

    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Cities"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("cities".to_string(), to_json(&docs));
    Ok(hb.render_response_ext("cities/list", &data, "html")?)
}

#[instrument]
pub async fn show(req: Request<State>) -> tide::Result {
    let state = &req.state();
    let db = &state.client.database("test");
    let id = req.param::<String>("city_id")?;

    let object_id = ObjectId::with_string(&id).unwrap();
    let filter = doc! { "_id": object_id };

    let doc = City::find_one(&db.clone(), filter, None).await?;
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Cities"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("city".to_string(), to_json(&doc));
    Ok(hb.render_response_ext("cities/show", &data, "html")?)
}

#[instrument]
pub async fn edit(req: Request<State>) -> tide::Result {
    let state = &req.state();
    let db = &state.client.database("test");
    let id = req.param::<String>("city_id")?;

    let object_id = ObjectId::with_string(&id).unwrap();
    let filter = doc! { "_id": object_id };

    let doc = City::find_one(&db.clone(), filter, None).await?;
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Cities"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    data.insert(
        "action".to_string(),
        to_json(format!("/cities/{}/edit", id)),
    );
    data.insert("city".to_string(), to_json(&doc));
    Ok(hb.render_response_ext("cities/form", &data, "html")?)
}

#[instrument]
pub async fn new(req: Request<State>) -> tide::Result {
    let hb = &req.state().registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("New City"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    data.insert("action".to_string(), to_json("/cities/new"));
    Ok(hb.render_response_ext("cities/form", &data, "html")?)
}

#[instrument]
pub async fn create(mut req: Request<State>) -> tide::Result {
    let mut city: City = req.body_form().await?;
    let state = &req.state();
    let db = &state.client.database("test");
    city.save(&db.clone(), None).await?;

    let city_id = city.id.unwrap();

    Ok(tide::Redirect::new(format!("/cities/{}", city_id.to_hex())).into())
}

#[instrument]
pub async fn update(mut req: Request<State>) -> tide::Result {
    let mut city: City = req.body_form().await?;
    let state = &req.state();
    let db = &state.client.database("test");
    let id = req.param::<String>("city_id")?;
    city.id = Some(ObjectId::with_string(&id).unwrap());

    city.save(&db.clone(), None).await?;

    let city_id = city.id.unwrap();

    Ok(tide::Redirect::new(format!("/cities/{}", city_id.to_hex())).into())
}
