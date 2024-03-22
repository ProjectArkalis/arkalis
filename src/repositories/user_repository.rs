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

pub async fn user_update_recovery_key(
    conn: &DatabaseConnection,
    user: User,
) -> Result<(), ApplicationError> {
    sqlx::query("update users set recovery_key = ? where id = ?")
        .bind(user.recovery_key)
        .bind(user.id)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(())
}

pub async fn user_get_by_recovery_key(
    conn: &DatabaseConnection,
    recovery_key: String,
) -> Result<User, ApplicationError> {
    let result = sqlx::query_as("select * from users where recovery_key = ?")
        .bind(recovery_key)
        .fetch_optional(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?
        .ok_or(ApplicationError::NotFound)?;

    Ok(result)
}
