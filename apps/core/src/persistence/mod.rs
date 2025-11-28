use sqlx::SqlitePool;

const SCHEMA_SQL: &str = include_str!("schema.sql");

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Enable WAL mode for durability.
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(pool)
        .await?;

    for statement in SCHEMA_SQL.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() {
            continue;
        }
        sqlx::query(stmt).execute(pool).await?;
    }

    Ok(())
}
