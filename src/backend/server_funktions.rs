use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::{db::get_db, model::ItemSql};
use crate::Item;

#[server]
pub async fn get_item_list() -> Result<Vec<Item>, ServerFnError> {
    let db = get_db().await;

    let rows: Vec<ItemSql> = sqlx::query_as("SELECT * FROM items")
        .fetch_all(db)
        .await
        .unwrap();

    let mut v = vec![];

    for row in rows {
        let item = Item {
            id: row.id,
            name: row.name,
            mass: row.mass,
            unit: row.unit,
            experation: row.experation,
        };
        v.push(item)
    }
    Ok(v)
}

#[server]
pub async fn get_single_item(id: i64) -> Result<Item, ServerFnError> {
    let db = get_db().await;

    let rows: Vec<ItemSql> = sqlx::query_as("SELECT * FROM items WHERE id = ?1")
        .bind(&id)
        .fetch_all(db)
        .await
        .unwrap();

    if rows.len() == 0 {
        let msg = format!("Item id : {} Not found.", id);
        Err(ServerFnError::new(msg))
    } else {
        let item = Item {
            id: rows[0].id,
            name: rows[0].name.clone(),
            mass: rows[0].mass,
            unit: rows[0].unit.clone(),
            experation: rows[0].experation.clone(),
        };
        Ok(item)
    }
}

#[server]
pub async fn add_new_item(
    name: String,
    mass: i64,
    unit: String,
    experation: String,
) -> Result<i64, ServerFnError> {
    let db = get_db().await;

    let result =
        sqlx::query("INSERT INTO items (name, mass, unit, experation) VALUES (?1, ?2, ?3, ?4)")
            .bind(&name)
            .bind(&mass)
            .bind(&unit)
            .bind(&experation)
            .execute(db)
            .await
            .unwrap();

    Ok(result.last_insert_rowid())
}

#[server]
pub async fn remove_item(id: i64) -> Result<(), ServerFnError> {
    let db = get_db().await;

    let rows: Vec<ItemSql> = sqlx::query_as("SELECT * FROM items WHERE id = ?1")
        .bind(&id)
        .fetch_all(db)
        .await
        .unwrap();
    let rows: Vec<ItemSql> = sqlx::query_as("SELECT * FROM items WHERE id = ?1")
        .bind(&id)
        .fetch_all(db)
        .await
        .unwrap();

    if rows.len() == 0 {
        let msg = format!("Item id : {} Not found.", id);
        Err(ServerFnError::new(msg))
    } else {
        sqlx::query("DELETE FROM items WHERE id = ?1")
            .bind(&id)
            .execute(db)
            .await
            .unwrap();
        Ok(())
    }
}

#[server]
pub async fn update_item_quantity(id: i64, masss: String) -> Result<(), ServerFnError> {
    let db = get_db().await;
    println!("{}", masss);
    let mut mass: i64;
    mass = masss.parse().unwrap();

    let rows: Vec<ItemSql> = sqlx::query_as("SELECT * FROM items WHERE id = ?1")
        .bind(&id)
        .fetch_all(db)
        .await
        .unwrap();

    if rows.len() == 0 {
        let msg = format!("Item id : {} Not Found.", id);
        Err(ServerFnError::new(msg))
    } else {
        sqlx::query("UPDATE items SET mass = ?1 WHERE id = ?2")
            .bind(&mass)
            .bind(&id)
            .execute(db)
            .await
            .unwrap();
        Ok(())
    }
}
