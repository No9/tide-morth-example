//mod dal;

use super::*;
use handlebars::to_json;
use serde_json::value::Map;
use tide::Request;
use tide_mongodb_dal;

pub async fn index(req: Request<State>) -> tide::Result {
    let state = &req.state();

    let client = &state.client;
    let docs = match tide_mongodb_dal::get(client, "test", "articles").await {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            Vec::new()
        }
    };
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("hello tide!"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("articles".to_string(), to_json(&docs));
    Ok(hb.render_response_ext("articles/list", &data, "html")?)
}
