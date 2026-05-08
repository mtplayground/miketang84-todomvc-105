use crate::server::todos::AddTodo;
use leptos::html;
use leptos::prelude::*;
use leptos::server::ServerAction;
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
        <Title text="TodoMVC • Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=TodoShell/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn TodoShell() -> impl IntoView {
    let add_todo = ServerAction::<AddTodo>::new();
    let input_ref = NodeRef::<html::Input>::new();
    let is_submitting = add_todo.pending();

    let submit_new_todo = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() != "Enter" {
            return;
        }

        let Some(input) = input_ref.get() else {
            return;
        };

        let title = input.value().trim().to_owned();

        if title.is_empty() {
            input.set_value("");
            return;
        }

        add_todo.dispatch(AddTodo { title });
        input.set_value("");
    };

    view! {
        <>
            <section class="todoapp">
                <header class="header">
                    <h1>"todos"</h1>
                    <input
                        node_ref=input_ref
                        class="new-todo"
                        placeholder="What needs to be done?"
                        autofocus=true
                        autocomplete="off"
                        prop:disabled=move || is_submitting.get()
                        on:keydown=submit_new_todo
                    />
                </header>
                <section class="main">
                    <input id="toggle-all" class="toggle-all" type="checkbox" />
                    <label for="toggle-all">"Mark all as complete"</label>
                    <ul class="todo-list">
                        <li class="completed">
                            <div class="view">
                                <input class="toggle" type="checkbox" checked=true />
                                <label>"Taste JavaScript"</label>
                                <button class="destroy"></button>
                            </div>
                            <input class="edit" value="Create a TodoMVC template" />
                        </li>
                        <li>
                            <div class="view">
                                <input class="toggle" type="checkbox" />
                                <label>"Buy a unicorn"</label>
                                <button class="destroy"></button>
                            </div>
                            <input class="edit" value="Rule the web" />
                        </li>
                    </ul>
                </section>
                <footer class="footer">
                    <span class="todo-count">
                        <strong>"0"</strong>
                        " item left"
                    </span>
                    <ul class="filters">
                        <li>
                            <a class="selected" href="#/">"All"</a>
                        </li>
                        <li>
                            <a href="#/active">"Active"</a>
                        </li>
                        <li>
                            <a href="#/completed">"Completed"</a>
                        </li>
                    </ul>
                    <button class="clear-completed">"Clear completed"</button>
                </footer>
            </section>

            <footer class="info">
                <p>"Double-click to edit a todo"</p>
                <p>"Built with Leptos"</p>
                <p>
                    <a href="https://todomvc.com/">"TodoMVC"</a>
                </p>
            </footer>
        </>
    }
}
