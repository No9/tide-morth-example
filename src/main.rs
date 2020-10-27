pub mod models;

extern crate serde_json;

use async_std::sync::Arc;
use handlebars::Handlebars;
use mongodb::{options::ClientOptions, Client};
use tide_handlebars::prelude::*;

use opentelemetry::api::Provider;
use opentelemetry::sdk;
use tracing_subscriber::prelude::*;

mod routes;

#[derive(Clone, Debug)]
pub struct State {
    registry: Arc<Handlebars<'static>>,
    client: Arc<Client>,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut client_options = match ClientOptions::parse("mongodb://127.0.0.1:27017").await {
        Ok(c) => c,
        Err(e) => panic!("Client Options Failed: {}", e),
    };
    //.with_env_filter("async_fn=trace")

    // match init_tracer() {
    //     Ok(o) => o,
    //     Err(e) => panic!("tracing failed {}", e),
    // };
    // Manually set an option.
    client_options.app_name = Some("MoRTH".to_string());

    // Get a handle to the deployment.
    let client = match Client::with_options(client_options) {
        Ok(c) => c,
        Err(e) => panic!("Client Creation Failed: {}", e),
    };
    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "./views").unwrap();

    let engine = State {
        registry: Arc::new(hb),
        client: Arc::new(client),
    };

    let mut app = tide::with_state(engine);
    app.at("/").get(tide::Redirect::new("/welcome"));

    app.at("/welcome").get(routes::welcome);

    let mut cities = app.at("/cities");
    cities.get(routes::cities::index);
    cities
        .at("/new")
        .get(routes::cities::new)
        .post(routes::cities::create);

    cities.at("/:city_id").get(routes::cities::show);
    cities
        .at("/:city_id/edit")
        .get(routes::cities::edit)
        .post(routes::cities::update);
    cities
        .at("/:city_id/delete")
        .get(routes::cities::delete);

    app.at("/public").serve_dir("public/")?;
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

// fn init_tracer() -> Result<(), Box<dyn std::error::Error>> {
//     let exporter = opentelemetry_jaeger::Exporter::builder()
//         .with_agent_endpoint("127.0.0.1:6831".parse().unwrap())
//         .with_process(opentelemetry_jaeger::Process {
//             service_name: "morth_example".to_string(),
//             tags: Vec::new(),
//         })
//         .init()?;
//     let provider = sdk::Provider::builder()
//         .with_simple_exporter(exporter)
//         .with_config(sdk::Config {
//             default_sampler: Box::new(sdk::Sampler::Always),
//             ..Default::default()
//         })
//         .build();
//     let tracer = provider.get_tracer("tracing");

//     let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
//     tracing_subscriber::registry()
//         .with(opentelemetry)
//         .try_init()?;

//     Ok(())
// }
