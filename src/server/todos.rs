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

#[server]
pub async fn add_todo(title: String) -> Result<Todo, ServerFnError> {
    use crate::state::AppState;
    use sqlx::query_as;

    let title = title.trim().to_owned();

    if title.is_empty() {
        return Err(ServerFnError::ServerError(
            "todo title cannot be empty".to_string(),
        ));
    }

    let app_state = expect_context::<AppState>();
    let todo = query_as::<_, Todo>(
        r#"
            INSERT INTO todos (title)
            VALUES (?1)
            RETURNING id, title, completed, created_at, updated_at
        "#,
    )
    .bind(title)
    .fetch_one(&app_state.pool)
    .await?;

    Ok(todo)
}

#[server]
pub async fn toggle_todo(
    id: i64,
    completed: bool,
) -> Result<Todo, ServerFnError> {
    use crate::state::AppState;
    use sqlx::query_as;

    let app_state = expect_context::<AppState>();
    let todo = query_as::<_, Todo>(
        r#"
            UPDATE todos
            SET completed = ?2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            RETURNING id, title, completed, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(completed)
    .fetch_optional(&app_state.pool)
    .await?;

    todo.ok_or_else(|| {
        ServerFnError::ServerError(format!("todo with id `{id}` was not found"))
    })
}
