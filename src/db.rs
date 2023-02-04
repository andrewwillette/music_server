use rusqlite::{Connection, Result, Statement};

pub const DB_PATH: &str = "./music_server.db";

#[derive(Debug)]
pub struct SoundcloudUrl {
    _id: i32,
    pub url: String,
}

pub struct DbContext<'a> {
    pub conn: &'a Connection,
    pub insert_soundcloud_url_statement: Option<Statement<'a>>,
    pub get_soudncloud_urls_statement: Option<Statement<'a>>,
}

impl<'a> DbContext<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        return DbContext {
            conn,
            insert_soundcloud_url_statement: None,
            get_soudncloud_urls_statement: None,
        };
    }

    pub fn init_soundcloud_db(&mut self) -> Result<()> {
        const SOUNDCLOUD_URL_TABLE_EXISTS_QUERY: &str =
            "SELECT name FROM sqlite_master WHERE type='table' AND name='soundcloudurl';";
        const CREATE_SOUNDCLOUD_URL_TABLE_QUERY: &str = "CREATE TABLE soundcloudurl (
        id    INTEGER PRIMARY KEY,
        url   TEXT NOT NULL,
        data  BLOB)";
        let stmt = Some(self.conn.prepare(SOUNDCLOUD_URL_TABLE_EXISTS_QUERY)?);
        let row: Result<String> = stmt.unwrap().query_row([], |row| return row.get(0));
        match row {
            Ok(_) => {}
            Err(_) => {
                self.conn.execute(CREATE_SOUNDCLOUD_URL_TABLE_QUERY, ())?;
            }
        }
        return Ok(());
    }

    pub fn insert_soundcloud_url(&mut self, url: &String) -> Result<i64> {
        if let None = &self.insert_soundcloud_url_statement {
            let stmt = self
                .conn
                .prepare("INSERT INTO soundcloudurl (url) VALUES (:url)")?;
            self.insert_soundcloud_url_statement = Some(stmt);
        };
        self.insert_soundcloud_url_statement
            .as_mut()
            .unwrap()
            .execute(&[(":url", &url)])?;
        return Ok(self.conn.last_insert_rowid());
    }

    pub fn get_soundcloud_urls(&mut self) -> Result<Vec<SoundcloudUrl>> {
        if let None = &self.get_soudncloud_urls_statement {
            let stmt = self.conn.prepare("SELECT id, url FROM soundcloudurl")?;
            self.get_soudncloud_urls_statement = Some(stmt);
        };
        let soundcloud_url_iter = self
            .get_soudncloud_urls_statement
            .as_mut()
            .unwrap()
            .query_map([], |row| {
                Ok(SoundcloudUrl {
                    _id: row.get(0)?,
                    url: row.get(1)?,
                })
            })?;
        let mut soundcloud_urls = Vec::new();
        for soundcloud_url in soundcloud_url_iter {
            soundcloud_urls.push(soundcloud_url?);
        }
        Ok(soundcloud_urls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_context() {
        let conn = Connection::open(DB_PATH).unwrap();
        let mut db_context = DbContext::new(&conn);
        let insert_row = db_context.insert_soundcloud_url(&String::from("testurl"));
        assert!(insert_row.is_ok());
        let soundcloud_urls = db_context.get_soundcloud_urls();
        assert!(soundcloud_urls.is_ok());
    }

    #[test]
    fn test_init_soundcloud_db() {
        let conn = Connection::open(DB_PATH).unwrap();
        let mut db_context = DbContext::new(&conn);
        let result = db_context.init_soundcloud_db();
        assert!(result.is_ok());
    }
}
