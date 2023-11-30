mod app;

use crate::app::App;
use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use when3meet::configuration;
use when3meet::startup::run;
use when3meet::telemetry::{get_subscriber, init_subscriber};

use actix_files::Files;
use actix_web::{get, middleware, App, HttpServer, Responder};
use leptos::{get_configuration, view};
use leptos_actix::{generate_route_list, LeptosRoutes};

#[get("/style.css")]
async fn css() -> impl Responder {
    actix_files::NamedFile::open_async("target/site/pkg/when3meet.css").await
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("when3meet".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!().run(&db_pool).await?;

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?
}
