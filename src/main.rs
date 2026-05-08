#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use miketang84_todomvc_105::app::{shell, App};
    use std::{env, net::SocketAddr};
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

    fn site_addr_from_env() -> Result<Option<SocketAddr>, Box<dyn std::error::Error + Send + Sync>> {
        match env::var("LEPTOS_SITE_ADDR") {
            Ok(value) if value.trim().is_empty() => Ok(None),
            Ok(value) => Ok(Some(value.parse()?)),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(error) => Err(Box::new(error)),
        }
    }

    init_tracing()?;

    let configuration = get_configuration(None)?;
    let mut leptos_options = configuration.leptos_options;

    if let Some(site_addr) = site_addr_from_env()? {
        leptos_options.site_addr = site_addr;
    }

    let site_addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let static_file_handler = leptos_axum::file_and_error_handler(shell);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(static_file_handler)
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    info!(%site_addr, "listening");
    let listener = tokio::net::TcpListener::bind(site_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
