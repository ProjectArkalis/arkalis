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
