#[cfg(feature = "server")]
#[derive(sqlx::FromRow)]
pub struct ItemSql {
    pub id: i64,
    pub name: String,
    pub mass: i64,
    pub unit: String,
    pub experation: String,
}
