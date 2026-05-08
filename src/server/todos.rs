use crate::domain::{Filter, Todo, TodoCounts};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
async fn current_todo_counts(
    pool: &sqlx::SqlitePool,
) -> Result<TodoCounts, sqlx::Error> {
    sqlx::query_as::<_, TodoCounts>(
        r#"
            SELECT
                SUM(CASE WHEN completed = 0 THEN 1 ELSE 0 END) AS active,
                COUNT(*) AS total
            FROM todos
        "#,
    )
    .fetch_one(pool)
    .await
}

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

#[server]
pub async fn edit_todo(
    id: i64,
    title: String,
) -> Result<Option<Todo>, ServerFnError> {
    use crate::state::AppState;
    use sqlx::query_as;

    let title = title.trim().to_owned();
    let app_state = expect_context::<AppState>();

    if title.is_empty() {
        let deleted = query_as::<_, Todo>(
            r#"
                DELETE FROM todos
                WHERE id = ?1
                RETURNING id, title, completed, created_at, updated_at
            "#,
        )
        .bind(id)
        .fetch_optional(&app_state.pool)
        .await?;

        return deleted
            .map(|_| None)
            .ok_or_else(|| {
                ServerFnError::ServerError(format!(
                    "todo with id `{id}` was not found"
                ))
            });
    }

    let todo = query_as::<_, Todo>(
        r#"
            UPDATE todos
            SET title = ?2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            RETURNING id, title, completed, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(title)
    .fetch_optional(&app_state.pool)
    .await?;

    todo.map(Some).ok_or_else(|| {
        ServerFnError::ServerError(format!("todo with id `{id}` was not found"))
    })
}

#[server]
pub async fn delete_todo(id: i64) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use sqlx::query;

    let app_state = expect_context::<AppState>();
    let result = query(
        r#"
            DELETE FROM todos
            WHERE id = ?1
        "#,
    )
    .bind(id)
    .execute(&app_state.pool)
    .await?;

    Ok(result.rows_affected())
}

#[server]
pub async fn toggle_all(completed: bool) -> Result<TodoCounts, ServerFnError> {
    use crate::state::AppState;
    use sqlx::query;

    let app_state = expect_context::<AppState>();

    query(
        r#"
            UPDATE todos
            SET completed = ?1,
                updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(completed)
    .execute(&app_state.pool)
    .await?;

    Ok(current_todo_counts(&app_state.pool).await?)
}

#[server]
pub async fn clear_completed() -> Result<TodoCounts, ServerFnError> {
    use crate::state::AppState;
    use sqlx::query;

    let app_state = expect_context::<AppState>();

    query(
        r#"
            DELETE FROM todos
            WHERE completed = 1
        "#,
    )
    .execute(&app_state.pool)
    .await?;

    Ok(current_todo_counts(&app_state.pool).await?)
}
