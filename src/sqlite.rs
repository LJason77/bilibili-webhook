use rusqlite::{named_params, Connection, Result};
use serde::{Deserialize, Serialize};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // 创建内容表
        let contents_sql = "CREATE TABLE IF NOT EXISTS contents (
                    id        INTEGER PRIMARY KEY,
                    source_id INTEGER NOT NULL,
                    link      TEXT NOT NULL,
                    title     TEXT NOT NULL
                )";
        conn.execute(contents_sql, [])?;
        // 创建索引以提高查询性能
        conn.execute("CREATE UNIQUE INDEX IF NOT EXISTS idx_content_link ON contents (link)", [])?;

        let sources_sql = "CREATE TABLE IF NOT EXISTS sources (
                    id        INTEGER PRIMARY KEY,
                    link      TEXT NOT NULL,
                    title     TEXT NOT NULL
                )";
        conn.execute(sources_sql, [])?;

        Ok(Self { conn })
    }
}

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

impl Source {
    const INSERT_SQL: &str = "INSERT INTO sources (link, title) VALUES (?1, ?2)";
    const QUERY_SQL: &str = "SELECT id FROM sources WHERE link = ?1";

    pub fn insert(conn: &Connection, link: &str, title: &str) -> Result<u32> {
        let mut stmt = conn.prepare_cached(Self::QUERY_SQL)?;

        if !stmt.exists([link])? {
            let mut stmt = conn.prepare_cached(Self::INSERT_SQL)?;
            stmt.execute([link, title])?;
        }

        let mut rows = stmt.query([link])?;
        if let Some(row) = rows.next()? {
            return row.get(0);
        }
        Ok(0)
    }
}

impl Content {
    const INSERT_SQL: &str = "INSERT INTO contents (source_id, link, title) VALUES (:source_id, :link, :title)";
    const QUERY_SQL: &str = "SELECT id, source_id, link, title FROM contents WHERE link = ?1";

    pub fn insert(conn: &Connection, source_id: u32, link: &str, title: &str) -> Result<()> {
        if !Self::exists(conn, link)? {
            let mut stmt = conn.prepare_cached(Self::INSERT_SQL)?;
            stmt.execute(named_params! { ":source_id": source_id, ":link": link, ":title": title })?;
        }

        Ok(())
    }

    pub fn exists(conn: &Connection, link: &str) -> Result<bool> {
        let mut stmt = conn.prepare_cached(Self::QUERY_SQL)?;
        stmt.exists([link])
    }
}
