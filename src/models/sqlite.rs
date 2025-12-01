use log::{error, info};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Source {
    pub id: u32,
    pub link: String,
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct Content {
    pub id: u32,
    pub source_id: u32,
    pub link: String,
    pub title: String,
}

pub fn open() -> Result<Connection, Box<dyn std::error::Error>> {
    let connection = Connection::open("config/date.db").map_err(|e| format!("打开 config/date.db 失败：{e}"))?;

    let contents_sql = "create table if not exists contents (
                    id        INTEGER PRIMARY KEY,
                    source_id INTEGER NOT NULL,
                    link      TEXT NOT NULL,
                    title     TEXT NOT NULL
                )";
    connection.execute(contents_sql, []).map_err(|e| format!("创建 contents 表失败：{e}"))?;

    let sources_sql = "create table if not exists sources (
                    id        INTEGER PRIMARY KEY,
                    link      TEXT NOT NULL,
                    title     TEXT NOT NULL
                )";
    connection.execute(sources_sql, []).map_err(|e| format!("创建 sources 表失败：{e}"))?;

    Ok(connection)
}

impl Source {
    pub fn insert(connection: &Connection, link: &str, title: &str) -> Result<Self> {
        let sql = "insert into sources (link, title) VALUES (?1, ?2)";
        match connection.execute(sql, params![link, title]) {
            Ok(_) => {
                info!("已添加订阅源：[{title}]");
                Self::query_where(connection, link)
            }
            Err(error) => {
                error!("插入 sources 表失败：{error}");
                Err(error)
            }
        }
    }

    pub fn query_where(connection: &Connection, link: &str) -> Result<Self> {
        let sql = "SELECT id, link, title FROM sources WHERE link = ?";
        connection.query_row(sql, params![link], |row| Ok(Self { id: row.get(0)?, link: row.get(1)?, title: row.get(2)? }))
    }
}

impl Content {
    pub fn insert(connection: &Connection, source_id: u32, link: &str, title: &str) -> Result<Self> {
        let sql = "insert into contents (source_id, link, title) VALUES (?1, ?2, ?3)";
        if let Err(error) = connection.execute(sql, params![source_id, link, title]) {
            error!("插入 contents 表失败：{error}");
        }
        Self::query_where(connection, link)
    }

    pub fn query_where(connection: &Connection, link: &str) -> Result<Self> {
        let sql = "SELECT id, source_id, link, title FROM contents WHERE link = ?";
        connection.query_row(sql, params![link], |row| Ok(Self { id: row.get(0)?, source_id: row.get(1)?, link: row.get(2)?, title: row.get(3)? }))
    }
}
