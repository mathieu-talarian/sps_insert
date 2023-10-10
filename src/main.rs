mod outputs;
mod write_sps;

use actix_web::{get, web, HttpRequest, HttpResponse, HttpServer, Responder};

use crate::outputs::Outputs;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use outputs::{Create, Output};
use std::f32::consts::{E, PI};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

#[macro_use]
extern crate tracing;

#[get("/")]
async fn hello() -> impl Responder {
    info!("Hello world request");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo")]
async fn echo(http_req: HttpRequest, req_body: String) -> impl Responder {
    let mut body: Outputs = Outputs::new();
    body.push(Output::create(1));
    body.push(Output::create(2));
    body.push(Output::create(3));
    body.push(Output::create(12.3));
    body.push(Output::create(15.99));
    body.push(Output::create(1f32 / 3f32));
    body.push(Output::create(PI));
    body.push(Output::create(E));
    web::Json(body)
}

fn init_telemetry() {
    let app_name = "tracing-actix-web-demo";

    // Start a new Jaeger trace pipeline.
    // Spans are exported in batch - recommended setup for a production application.
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(app_name)
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer.");

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    // Create a `tracing` layer using the Jaeger tracer
    // Create a `tracing` layer to emit spans as structured logs to stdout
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
    // Combined them all together in a `tracing` subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    info!(message = "Validator", credentials = ?credentials);
    Ok(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    write_sps::write_sps().await?;
    Ok(())
    // init_telemetry();
    //
    // HttpServer::new(|| {
    //     // let middleware = HttpAuthentication::bearer(validator);
    //
    //     App::new()
    //         // .wrap(middleware)
    //         .wrap(TracingLogger::default())
    //         .service(hello)
    //         .service(echo)
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
}
