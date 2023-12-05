use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::{Files};
        use actix_web::*;
        use crate::components::App;
        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};

        #[tokio::main]
        async fn main() -> std::io::Result<()> {
            let conf = get_configuration(None).await.unwrap();

            let addr = conf.leptos_options.site_addr;

            let routes = generate_route_list(App);

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;
                App::new()
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
                    .service(Files::new("/", site_root))
                    .wrap(middleware::Compress::default())
            })
            .bind(addr)?
            .run()
            .await?;

        Ok(())
        }
    } else {
        fn main() {}
    }
}
