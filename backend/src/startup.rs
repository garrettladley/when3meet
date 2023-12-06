use crate::routes::{create_meeting, create_user, health_check, read_meeting, update_user};
use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://[::1]:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["content-type"]);
        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .service(
                web::scope("/meeting")
                    .route("/create", web::post().to(create_meeting))
                    .route("/{id}", web::get().to(read_meeting)),
            )
            .service(
                web::scope("/user")
                    .route("/create/{meeting_id}", web::post().to(create_user))
                    .route("/update/{user_id}", web::post().to(update_user)),
            )
            .route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
