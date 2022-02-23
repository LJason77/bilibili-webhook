use log::info;
use rusqlite::{params, Connection, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Source {
    pub id: u32,
    pub link: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct Content {
    pub id: u32,
    pub source_id: u32,
    pub link: String,
    pub title: String,
}

#[must_use]
pub fn open() -> Connection {
    let connection = Connection::open("config/date.db").expect("打开 config/date.db 失败");
    connection
        .execute(
            "create table if not exists contents (
                    id        INTEGER PRIMARY KEY,
                    source_id INTEGER NOT NULL,
                    link      TEXT NOT NULL,
                    title     TEXT NOT NULL
                )",
            [],
        )
        .expect("创建 contents 表失败");
    connection
        .execute(
            "create table if not exists sources (
                    id        INTEGER PRIMARY KEY,
                    link      TEXT NOT NULL,
                    title     TEXT NOT NULL
                )",
            [],
        )
        .expect("创建 sources 表失败");
    connection
}

impl Source {
    /// # Panics
    /// 如果插入失败，则返回错误信息
    pub fn insert(connection: &Connection, link: &str, title: &str) -> Self {
        connection
            .execute("insert into sources (link, title) VALUES (?1, ?2)", params![link, title])
            .expect("插入 sources 表失败");
        info!("已添加订阅源：[{}]", title);
        Self::query_where(connection, link).unwrap()
    }

    /// # Errors
    /// 如果查询失败，则返回错误信息
    pub fn query_where(connection: &Connection, link: &str) -> Result<Self> {
        connection.query_row(
            "SELECT id, link, title FROM sources WHERE link = ?",
            params![link],
            |row| Ok(Self { id: row.get(0)?, link: row.get(1)?, title: row.get(2)? }),
        )
    }
}

impl Content {
    /// # Panics
    /// 如果插入失败，则返回错误信息
    pub fn insert(connection: &Connection, source_id: u32, link: &str, title: &str) -> Self {
        connection
            .execute(
                "insert into contents (source_id, link, title) VALUES (?1, ?2, ?3)",
                params![source_id, link, title],
            )
            .expect("插入 contents 表失败");
        Self::query_where(connection, link).unwrap()
    }

    /// # Errors
    /// 如果查询失败，则返回错误信息
    pub fn query_where(connection: &Connection, link: &str) -> Result<Self> {
        connection.query_row(
            "SELECT id, source_id, link, title FROM contents WHERE link = ?",
            params![link],
            |row| {
                Ok(Self {
                    id: row.get(0)?,
                    source_id: row.get(1)?,
                    link: row.get(2)?,
                    title: row.get(3)?,
                })
            },
        )
    }
}
