use once_cell::sync::OnceCell;
use sqlx::sqlite::SqlitePool;

pub static DB: OnceCell<SqlitePool> = OnceCell::new();
