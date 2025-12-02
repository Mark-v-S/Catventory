#[cfg(feature = "server")]
use sqlx::{Executor, Pool, Sqlite};
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

#[cfg(feature = "server")]
async fn db() -> Pool<Sqlite> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite")
        .await
        .unwrap();

    pool.execute(
        "CREATE TABLE IF NOT EXISTS items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        mass INTEGER,
        unit TEXT,
        experation TEXT
        )",
    )
    .await
    .unwrap();

    pool
}

#[cfg(feature = "server")]
pub async fn get_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(db).await
}
