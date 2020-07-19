//mod dal;

use super::*;
use handlebars::to_json;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
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
    data.insert("title".to_string(), to_json("Articles"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("articles".to_string(), to_json(&docs));
    Ok(hb.render_response_ext("articles/list", &data, "html")?)
}

pub async fn show(req: Request<State>) -> tide::Result {
    let state = &req.state();

    let client = &state.client;
    let id = req.param::<String>("article_id")?;
    //let id = req.param("article_id")?;

    let doc = match tide_mongodb_dal::get_by_id(client, "test", "articles", &id).await {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            Document::new()
        }
    };
    println!("show:{}", doc);
    let hb = &state.registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("Articles"));
    data.insert("parent".to_string(), to_json("layouts/main"));

    data.insert("article".to_string(), to_json(&doc));
    Ok(hb.render_response_ext("articles/show", &data, "html")?)
}

pub async fn new(req: Request<State>) -> tide::Result {
    let hb = &req.state().registry;
    let mut data = Map::new();
    data.insert("title".to_string(), to_json("New Article"));
    data.insert("parent".to_string(), to_json("layouts/main"));
    data.insert("action".to_string(), to_json("/articles/new"));
    Ok(hb.render_response_ext("articles/form", &data, "html")?)
}

#[derive(Serialize, Deserialize)]
struct PartialArticle {
    title: String,
    text: String,
}
pub async fn create(mut req: Request<State>) -> tide::Result {
    let article: PartialArticle = req.body_form().await?;
    let doc = doc! { "title" : article.title, "text": article.text };
    let client = &req.state().client;

    let result = tide_mongodb_dal::insert_one(client, "test", "articles", doc)
        .await
        .expect("Insert Failed");

    let article_id = result.inserted_id.as_object_id().unwrap();

    Ok(tide::Redirect::new(format!("/articles/{}", article_id.to_hex())).into())

    // let hb = &req.state().registry;

    // let mut data = Map::new();
    // data.insert("title".to_string(), to_json("New Article Reponse"));
    // data.insert("parent".to_string(), to_json("layouts/main"));
    // Ok(hb.render_response_ext("articles/form", &data, "html")?)
    // let db = &request.state().db;
    // let mut tx = db.begin().await?;
    // let article: PartialArticle = utils::deserialize_body(&mut request).await?;
    // let created = article.create().execute(&mut tx).await?;

    // if created == 1 {
    //     let (last_id,) = Article::last_id().fetch_one(&mut tx).await?;
    //     Ok(tide::Redirect::new(format!("/articles/{}", last_id)).into())
    // } else {
    //     Ok(ArticleForm::for_partial_article(&article).into())
    // }
}
