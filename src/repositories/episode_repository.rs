use crate::models::episode::Episode;
use crate::models::error::ApplicationError;
use crate::repositories::DatabaseConnection;

pub async fn episode_add(
    conn: &DatabaseConnection,
    episode: Episode,
) -> Result<(), ApplicationError> {
    sqlx::query("insert into episodes (id, name, cover_id, season_id, source_id, is_nsfw, sequence) values (?, ?, ?, ?, ?, ?, ?)")
        .bind(episode.id)
        .bind(episode.name)
        .bind(episode.cover_id)
        .bind(episode.season_id)
        .bind(episode.source_id)
        .bind(episode.is_nsfw)
        .bind(episode.sequence)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(())
}
