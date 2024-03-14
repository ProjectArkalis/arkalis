use crate::models::error::ApplicationError;
use crate::models::user::User;
use crate::repositories::DatabaseConnection;

pub async fn user_add(conn: &DatabaseConnection, user: User) -> Result<(), ApplicationError> {
    sqlx::query("insert into users values (?, ?, ?)")
        .bind(user.id)
        .bind(user.display_name)
        .bind(user.role as u8)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(())
}
