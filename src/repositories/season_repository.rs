use crate::models::error::ApplicationError;
use crate::models::season::Season;
use crate::repositories::DatabaseConnection;
use sqlx::Row;

pub async fn season_add(
    conn: &DatabaseConnection,
    season: Season,
) -> Result<u32, ApplicationError> {
    let id =
        sqlx::query("insert into seasons (name, cover_id, anime_id, sequence) values (?, ?, ?, ?)")
            .bind(season.name)
            .bind(season.cover_id)
            .bind(season.anime_id)
            .bind(season.sequence)
            .execute(&conn.connection)
            .await
            .map_err(|e| ApplicationError::UnknownError(e.into()))?
            .last_insert_id();

    Ok(id as u32)
}

pub async fn season_get_last_sequence(
    conn: &DatabaseConnection,
    anime_id: u32,
) -> Result<u16, ApplicationError> {
    let result = sqlx::query(
        "select sequence from seasons where anime_id = ? order by sequence desc limit 1",
    )
    .bind(anime_id)
    .fetch_optional(&conn.connection)
    .await
    .map_err(|e| ApplicationError::UnknownError(e.into()))?
    .ok_or(ApplicationError::NotFound)?;

    result
        .try_get("sequence")
        .map_err(|e| ApplicationError::UnknownError(e.into()))
}
