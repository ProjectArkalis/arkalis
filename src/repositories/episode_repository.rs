use crate::models::episode::Episode;
use crate::models::error::ApplicationError;
use crate::repositories::DatabaseConnection;

pub async fn episode_add(
    conn: &DatabaseConnection,
    episode: Episode,
) -> Result<(), ApplicationError> {
    sqlx::query("insert into episodes (id, name, cover_id, season_id, source_id, is_nsfw, sequence, is_hidden) values (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(episode.id)
        .bind(episode.name)
        .bind(episode.cover_id)
        .bind(episode.season_id)
        .bind(episode.source_id)
        .bind(episode.is_nsfw)
        .bind(episode.sequence)
        .bind(episode.is_hidden)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(())
}

pub async fn episode_get_by_id(
    conn: &DatabaseConnection,
    id: &str,
) -> Result<Episode, ApplicationError> {
    let result = sqlx::query_as("select * from episodes where id = ?")
        .bind(id)
        .fetch_one(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(result)
}

pub async fn episode_update(
    conn: &DatabaseConnection,
    episode: Episode,
) -> Result<(), ApplicationError> {
    sqlx::query("update episodes set cover_id = ?, lbry_media_id = ?, file_name = ?, sequence = ?, is_hidden = ? where id = ?")
        .bind(episode.cover_id)
        .bind(episode.lbry_media_id)
        .bind(episode.file_name)
        .bind(episode.sequence)
        .bind(episode.is_hidden)
        .bind(episode.id)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(())
}

pub async fn episode_get_by_season_and_source(
    conn: &DatabaseConnection,
    season_id: u32,
    source_id: u32,
) -> Result<Vec<Episode>, ApplicationError> {
    let result = sqlx::query_as("select * from episodes where season_id = ? and source_id = ? and lbry_media_id is not null and file_name is not null and is_hidden = false order by sequence")
        .bind(season_id)
        .bind(source_id)
        .fetch_all(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(result)
}
