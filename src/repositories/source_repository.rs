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
    let GetSourcesRequest { source_type } = filters;

    let query = Query::select()
        .columns([
            SourceQueryTable::Id,
            SourceQueryTable::Name,
            SourceQueryTable::SourceType,
            SourceQueryTable::Priority,
        ])
        .from(SourceQueryTable::Table)
        .conditions(
            filters.source_type.is_some(),
            move |q| {
                q.and_where(Expr::col(SourceQueryTable::SourceType).eq(source_type.unwrap()));
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