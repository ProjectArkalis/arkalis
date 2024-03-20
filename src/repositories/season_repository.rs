use sqlx::Row;

use crate::models::error::ApplicationError;
use crate::models::season::Season;
use crate::repositories::DatabaseConnection;

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

pub async fn season_get_by_anime(
    conn: &DatabaseConnection,
    anime_id: u32,
) -> Result<Vec<Season>, ApplicationError> {
    let result = sqlx::query_as("select * from seasons where anime_id = ? order by sequence")
        .bind(anime_id)
        .fetch_all(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(result)
}

pub async fn season_update(
    conn: &DatabaseConnection,
    season: Season,
) -> Result<(), ApplicationError> {
    sqlx::query("update seasons set name = ?, cover_id = ?, sequence = ? where id = ?")
        .bind(season.name)
        .bind(season.cover_id)
        .bind(season.sequence)
        .bind(season.id)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(())
}

pub async fn season_bu_id(
    conn: &DatabaseConnection,
    season_id: u32,
) -> Result<Season, ApplicationError> {
    let result = sqlx::query_as("select * from seasons where id = ?")
        .bind(season_id)
        .fetch_optional(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?
        .ok_or(ApplicationError::NotFound)?;

    Ok(result)
}
