use crate::arkalis_service::SearchAnimeRequest;
use crate::models::anime::Anime;
use crate::models::error::ApplicationError;
use crate::repositories::DatabaseConnection;
use chrono::DateTime;
use sea_query::{Expr, Iden, MysqlQueryBuilder, Query, SimpleExpr};
use std::fmt::Write;

enum AnimeQueryTable {
    Table,
    Id,
    Titles,
    TitleSearch,
    Synopsis,
    ThumbnailId,
    BannerId,
    IsHidden,
    IsNsfw,
    CreatedBy,
    CreatedAt,
    Genre,
    ReleaseDate,
    AnimeInLists,
}

impl Iden for AnimeQueryTable {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                AnimeQueryTable::Table => "animes",
                AnimeQueryTable::TitleSearch => "title_search",
                AnimeQueryTable::Synopsis => "synopsis",
                AnimeQueryTable::IsNsfw => "is_nsfw",
                AnimeQueryTable::Genre => "genre",
                AnimeQueryTable::ReleaseDate => "release_date",
                AnimeQueryTable::Titles => "titles",
                AnimeQueryTable::ThumbnailId => "thumbnail_id",
                AnimeQueryTable::BannerId => "banner_id",
                AnimeQueryTable::IsHidden => "is_hidden",
                AnimeQueryTable::CreatedBy => "created_by",
                AnimeQueryTable::CreatedAt => "created_at",
                AnimeQueryTable::AnimeInLists => "anime_in_lists",
                AnimeQueryTable::Id => "id",
            }
        )
        .unwrap()
    }
}

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

pub async fn anime_search(
    conn: &DatabaseConnection,
    filters: SearchAnimeRequest,
) -> Result<Vec<Anime>, ApplicationError> {
    let SearchAnimeRequest {
        title,
        synopsis,
        is_nsfw,
        genre,
        start_release_date,
        end_release_date,
    } = filters;

    let start_release_date = if let Some(dt) = start_release_date {
        Some(
            DateTime::from_timestamp(dt, 0).ok_or(ApplicationError::InvalidData(
                anyhow::Error::msg("start_release_date is invalid unix time"),
            ))?,
        )
    } else {
        None
    };

    let end_release_date = if let Some(dt) = end_release_date {
        Some(
            DateTime::from_timestamp(dt, 0).ok_or(ApplicationError::InvalidData(
                anyhow::Error::msg("end_release_date is invalid unix time"),
            ))?,
        )
    } else {
        None
    };

    let query = Query::select()
        .columns([
            AnimeQueryTable::Id,
            AnimeQueryTable::Titles,
            AnimeQueryTable::TitleSearch,
            AnimeQueryTable::Synopsis,
            AnimeQueryTable::ThumbnailId,
            AnimeQueryTable::BannerId,
            AnimeQueryTable::IsHidden,
            AnimeQueryTable::IsNsfw,
            AnimeQueryTable::CreatedBy,
            AnimeQueryTable::CreatedAt,
            AnimeQueryTable::Genre,
            AnimeQueryTable::ReleaseDate,
            AnimeQueryTable::AnimeInLists,
        ])
        .from(AnimeQueryTable::Table)
        .conditions(
            title.is_some(),
            move |q| {
                q.and_where(complete_like(AnimeQueryTable::TitleSearch, title.unwrap()));
            },
            |_| {},
        )
        .conditions(
            synopsis.is_some(),
            move |q| {
                q.and_where(complete_like(AnimeQueryTable::Synopsis, synopsis.unwrap()));
            },
            |_| {},
        )
        .conditions(
            is_nsfw.is_some(),
            move |q| {
                q.and_where(Expr::col(AnimeQueryTable::IsNsfw).eq(is_nsfw.unwrap()));
            },
            |_| {},
        )
        .conditions(
            genre.is_some(),
            |q| {
                q.and_where(Expr::col(AnimeQueryTable::Genre).eq(genre.unwrap()));
            },
            |_| {},
        )
        .conditions(
            start_release_date.is_some(),
            move |q| {
                q.and_where(
                    Expr::col(AnimeQueryTable::ReleaseDate).gte(start_release_date.unwrap()),
                );
            },
            |_| {},
        )
        .conditions(
            end_release_date.is_some(),
            move |q| {
                q.and_where(Expr::col(AnimeQueryTable::ReleaseDate).lte(end_release_date.unwrap()));
            },
            |_| {},
        )
        .to_string(MysqlQueryBuilder);

    let result = sqlx::query_as(&query)
        .fetch_all(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(result)
}

fn complete_like(col: AnimeQueryTable, value: String) -> SimpleExpr {
    Expr::col(col).like(format!("%{}%", value))
}
