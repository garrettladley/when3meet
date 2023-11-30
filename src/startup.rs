use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let conf = get_configuration(None).await.unwrap();
    let routes = generate_route_list(|| view! { <App/> });
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        let routes = &routes;
        App::new()
            .wrap(TracingLogger::default())
            .app_data(db_pool.clone())
            .service(css)
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                || view! { <App/> },
            )
            .service(Files::new("/", site_root))
            .wrap(middleware::Compress::default())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
