use crate::arkalis_service::GetSourcesRequest;
use crate::models::error::ApplicationError;
use crate::models::source::Source;
use crate::repositories::DatabaseConnection;
use sea_query::{Expr, Iden, MysqlQueryBuilder, Query};
use std::fmt::Write;

enum SourceQueryTable {
    Table,
    Id,
    Name,
    SourceType,
    Priority,
}

impl Iden for SourceQueryTable {
    fn unquoted(&self, s: &mut dyn Write) {
        let name = match self {
            SourceQueryTable::Table => "sources",
            SourceQueryTable::Id => "id",
            SourceQueryTable::Name => "name",
            SourceQueryTable::SourceType => "source_type",
            SourceQueryTable::Priority => "priority",
        };

        write!(s, "{}", name).unwrap()
    }
}

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

pub async fn source_get(
    conn: &DatabaseConnection,
    filters: GetSourcesRequest,
) -> Result<Vec<Source>, ApplicationError> {
    let GetSourcesRequest {
        source_type,
        name,
        priority,
    } = filters;

    let query = Query::select()
        .columns([
            SourceQueryTable::Id,
            SourceQueryTable::Name,
            SourceQueryTable::SourceType,
            SourceQueryTable::Priority,
        ])
        .from(SourceQueryTable::Table)
        .conditions(
            source_type.is_some(),
            move |q| {
                q.and_where(Expr::col(SourceQueryTable::SourceType).eq(source_type.unwrap()));
            },
            |_| {},
        )
        .conditions(
            name.is_some(),
            |q| {
                q.and_where(Expr::col(SourceQueryTable::Name).like(format!("%{}%", name.unwrap())));
            },
            |_| {},
        )
        .conditions(
            priority.is_some(),
            |q| {
                q.and_where(Expr::col(SourceQueryTable::Priority).eq(priority.unwrap() as u8));
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

pub async fn source_update(
    conn: &DatabaseConnection,
    source: Source,
) -> Result<(), ApplicationError> {
    sqlx::query("update sources set name = ?, source_type = ?, priority = ? where id = ?")
        .bind(source.name)
        .bind(source.source_type.bits())
        .bind(source.priority)
        .bind(source.id)
        .execute(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(())
}

pub async fn source_by_id(conn: &DatabaseConnection, id: u32) -> Result<Source, ApplicationError> {
    let result = sqlx::query_as("select * from sources where id = ?")
        .bind(id)
        .fetch_optional(&conn.connection)
        .await
        .map_err(|e| ApplicationError::UnknownError(e.into()))?
        .ok_or(ApplicationError::NotFound)?;

    Ok(result)
}

pub async fn sources_by_season_id(
    conn: &DatabaseConnection,
    season_id: u32,
) -> Result<Vec<Source>, ApplicationError> {
    let result = sqlx::query_as(
        "select s.* from episodes e join sources s on e.source_id = s.id where e.season_id = ? group by s.id, s.name, s.source_type, s.priority",
    )
    .bind(season_id)
    .fetch_all(&conn.connection)
    .await
    .map_err(|e| ApplicationError::UnknownError(e.into()))?;

    Ok(result)
}
