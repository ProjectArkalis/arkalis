use crate::models::error::ApplicationError;
use crate::models::source::Source;
use crate::repositories::DatabaseConnection;

pub async fn source_add(
    conn: &DatabaseConnection,
    source: Source,
) -> Result<u32, ApplicationError> {
    let id = sqlx::query("insert into sources (name, source_type, priority) values (?, ?, ?)")
        .bind(source.name)
        .bind(source.source_type.bits())
        .bind(source.priority)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?
        .last_insert_id();

    Ok(id as u32)
}
