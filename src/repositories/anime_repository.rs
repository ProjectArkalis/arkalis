use crate::models::anime::Anime;
use crate::models::error::ApplicationError;
use crate::repositories::DatabaseConnection;

pub async fn anime_add(conn: &DatabaseConnection, anime: Anime) -> Result<u32, ApplicationError> {
    let anime_in_list = anime.get_anime_in_anime_list_json()?;

    let id = sqlx::query("insert into animes (titles, title_search, synopsis, thumbnail_id, banner_id, is_hidden, is_nsfw, created_by, created_at, genre, release_date, anime_in_lists) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(anime.get_titles_json()?)
        .bind(anime.title_search)
        .bind(anime.synopsis)
        .bind(anime.thumbnail_id)
        .bind(anime.banner_id)
        .bind(anime.is_hidden)
        .bind(anime.is_nsfw)
        .bind(anime.created_by)
        .bind(anime.created_at)
        .bind(anime.genre.bits())
        .bind(anime.release_date)
        .bind(anime_in_list)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?
        .last_insert_id();

    Ok(id as u32)
}

pub async fn anime_get_by_id(
    conn: &DatabaseConnection,
    id: u32,
) -> Result<Anime, ApplicationError> {
    let result = sqlx::query_as("select * from animes where id = ?")
        .bind(id)
        .fetch_optional(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?
        .ok_or(ApplicationError::NotFound)?;

    Ok(result)
}
