#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use axum::{
        body::Body,
        http::Request,
        routing::get,
        Router,
    };
    use leptos::prelude::*;
    use leptos_axum::{
        generate_route_list, handle_server_fns_with_context, LeptosRoutes,
    };
    use miketang84_todomvc_105::{
        app::{shell, App},
        config::RuntimeEnv,
        state::AppState,
    };
    use tower_http::trace::TraceLayer;
    use tracing::info;
    use tracing_subscriber::EnvFilter;

    fn init_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let env_filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info,tower_http=info"))?;

        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(true)
            .compact()
            .try_init()?;

        Ok(())
    }
    let runtime_env = RuntimeEnv::load()?;

    init_tracing()?;

    let app_state = AppState::new(&runtime_env.database_url).await?;
    let configuration = get_configuration(None)?;
    let mut leptos_options = configuration.leptos_options;

    if let Some(site_addr) = runtime_env.site_addr {
        leptos_options.site_addr = site_addr;
    }

    let site_addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let static_file_handler = leptos_axum::file_and_error_handler(shell);
    let server_fn_handler = {
        let app_state = app_state.clone();

        move |request: Request<Body>| {
            let app_state = app_state.clone();

            async move {
                handle_server_fns_with_context(
                    move || provide_context(app_state.clone()),
                    request,
                )
                .await
            }
        }
    };

    let app = Router::new()
        .route(
            "/api/{*fn_name}",
            get(server_fn_handler.clone()).post(server_fn_handler),
        )
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(static_file_handler)
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    info!(
        %site_addr,
        database_configured = true,
        "listening"
    );
    let listener = tokio::net::TcpListener::bind(site_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
