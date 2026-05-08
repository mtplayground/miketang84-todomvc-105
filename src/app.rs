use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/miketang84-todomvc-105.css"/>
        <Title text="Hello, Leptos"/>

        <Router>
            <main class="shell">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let increment = move |_| count.update(|value| *value += 1);

    view! {
        <section class="hero">
            <p class="eyebrow">"Leptos + Axum SSR"</p>
            <h1>"Hello, Leptos"</h1>
            <p class="subtitle">
                "This page is rendered on the server and hydrated in the browser."
            </p>
            <button class="cta" on:click=increment>
                "Hydration clicks: "
                {count}
            </button>
        </section>
    }
}
