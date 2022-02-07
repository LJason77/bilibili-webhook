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

pub fn open() -> Connection {
    let connection = Connection::open("config/date.db").unwrap();
    connection
        .execute(
            "create table if not exists contents
(
    id        INTEGER PRIMARY KEY,
    source_id INTEGER NOT NULL,
    link      TEXT NOT NULL,
    title     TEXT NOT NULL
)",
            [],
        )
        .unwrap();
    connection
        .execute(
            "create table if not exists sources
(
    id        INTEGER PRIMARY KEY,
    link      TEXT NOT NULL,
    title     TEXT NOT NULL
)",
            [],
        )
        .unwrap();
    connection
}

impl Source {
    pub fn insert(connection: &Connection, link: &str, title: &str) -> Source {
        connection
            .execute(
                "insert into sources (link, title) VALUES (?1, ?2)",
                params![link, title],
            )
            .unwrap();
        log::info!("已添加订阅源：[{}]", title);
        Source::query_where(connection, link).unwrap()
    }

    pub fn query_where(connection: &Connection, link: &str) -> Result<Self> {
        connection.query_row(
            "SELECT id, link, title FROM sources WHERE link = ?",
            params![link],
            |row| {
                Ok(Source {
                    id: row.get(0)?,
                    link: row.get(1)?,
                    title: row.get(2)?,
                })
            },
        )
    }
}

impl Content {
    pub fn insert(connection: &Connection, source_id: u32, link: &str, title: &str) -> Content {
        connection
            .execute(
                "insert into contents (source_id, link, title) VALUES (?1, ?2, ?3)",
                params![source_id, link, title],
            )
            .unwrap();
        Content::query_where(connection, link).unwrap()
    }

    pub fn query_where(connection: &Connection, link: &str) -> Result<Self> {
        connection.query_row(
            "SELECT id, source_id, link, title FROM contents WHERE link = ?",
            params![link],
            |row| {
                Ok(Content {
                    id: row.get(0)?,
                    source_id: row.get(1)?,
                    link: row.get(2)?,
                    title: row.get(3)?,
                })
            },
        )
    }
}
