use cfg_if::cfg_if;
mod todo;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::{Files};
        use actix_web::*;
        use crate::todo::*;
        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("target/site/pkg/when3meet.css").await
        }

        #[tokio::main]
        async fn main() -> std::io::Result<()> {
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;

            let routes = generate_route_list(TodoApp);

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;
                App::new()
                    .service(css)
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), TodoApp)
                    .service(Files::new("/", site_root))
                    .wrap(middleware::Compress::default())
            })
            .bind(addr)?
            .run()
            .await?;

        Ok(())
        }
    } else {
        fn main() {
        }
    }
}
