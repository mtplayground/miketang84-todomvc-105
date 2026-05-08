use crate::domain::{Filter, Todo};
use leptos::prelude::*;

#[server]
pub async fn list_todos(filter: Filter) -> Result<Vec<Todo>, ServerFnError> {
    use crate::state::AppState;
    use sqlx::query_as;

    let app_state = expect_context::<AppState>();

    let todos = match filter {
        Filter::All => {
            query_as::<_, Todo>(
                r#"
                    SELECT id, title, completed, created_at, updated_at
                    FROM todos
                    ORDER BY created_at ASC, id ASC
                "#,
            )
            .fetch_all(&app_state.pool)
            .await?
        }
        Filter::Active => {
            query_as::<_, Todo>(
                r#"
                    SELECT id, title, completed, created_at, updated_at
                    FROM todos
                    WHERE completed = 0
                    ORDER BY created_at ASC, id ASC
                "#,
            )
            .fetch_all(&app_state.pool)
            .await?
        }
        Filter::Completed => {
            query_as::<_, Todo>(
                r#"
                    SELECT id, title, completed, created_at, updated_at
                    FROM todos
                    WHERE completed = 1
                    ORDER BY created_at ASC, id ASC
                "#,
            )
            .fetch_all(&app_state.pool)
            .await?
        }
    };

    Ok(todos)
}
