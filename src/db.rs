use rusqlite::{Connection, Result, Statement};

const DB_PATH: &str = "./music_server.db";

pub struct DbContext<'a> {
    pub conn: &'a Connection,
    pub insert_soundcloud_url_statement: Option<Statement<'a>>,
}

impl<'a> DbContext<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        return DbContext {
            conn: conn,
            insert_soundcloud_url_statement: None,
        };
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
            .execute_named(&[(":url", &url)])?;
        return Ok(self.conn.last_insert_rowid());
    }
}

#[derive(Debug)]
pub struct SoundcloudUrl {
    id: i32,
    url: String,
}

pub fn init_db() -> Result<()> {
    const SOUNDCLOUD_URL_TABLE_EXISTS_QUERY: &str =
        "SELECT name FROM sqlite_master WHERE type='table' AND name='soundcloudurl';";
    const CREATE_SOUNDCLOUD_URL_TABLE_QUERY: &str = "CREATE TABLE soundcloudurl (
        id    INTEGER PRIMARY KEY,
        url   TEXT NOT NULL,
        data  BLOB
    )";
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare(SOUNDCLOUD_URL_TABLE_EXISTS_QUERY)?;
    // let exists_rows = stmt.query_map((), |row| row.get(0))?;
    // conn.execute(SOUNDCLOUD_URL_TABLE_EXISTS_QUERY, ())?;
    // conn.execute(
    //     "CREATE TABLE soundcloudurl (
    //         id    INTEGER PRIMARY KEY,
    //         url   TEXT NOT NULL,
    //         data  BLOB
    //     )",
    //     (), // empty list of parameters.
    // )?;
    return Ok(());
}

pub fn insert_soundcloud_url(url: &String) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;
    conn.execute(
        "CREATE TABLE soundcloudurl (
            id    INTEGER PRIMARY KEY,
            url   TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let soundcloud_url = SoundcloudUrl {
        id: 0,
        url: url.to_string(),
    };
    conn.execute(
        "INSERT INTO soundcloudurl (url) VALUES (?1)",
        (&soundcloud_url.url,),
    )?;
    return Ok(());
}

pub fn get_soundcloud_urls() -> Result<Vec<SoundcloudUrl>> {
    let conn = Connection::open(DB_PATH)?;
    let mut stmt = conn.prepare("SELECT id, url FROM soundcloudurl")?;
    let soundcloud_url_iter = stmt.query_map([], |row| {
        Ok(SoundcloudUrl {
            id: row.get(0)?,
            url: row.get(1)?,
        })
    })?;
    let mut soundcloud_urls = Vec::new();
    for soundcloud_url in soundcloud_url_iter {
        soundcloud_urls.push(soundcloud_url?);
    }
    Ok(soundcloud_urls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db() {
        let url = String::from("https://soundcloud.com/thisismyurl");
        assert_eq!(insert_soundcloud_url(&url), Ok(()));
    }

    #[test]
    fn test_insert_soundcloud_url() {
        let url = String::from("https://soundcloud.com/thisismyurl");
        assert_eq!(insert_soundcloud_url(&url), Ok(()));
    }

    #[test]
    fn test_get_soundcloud_url() {
        let url = String::from("https://soundcloud.com/thisismyurl");
        assert_eq!(insert_soundcloud_url(&url), Ok(()));
        let soundcloud_urls = get_soundcloud_urls().unwrap();
        print!("{:?}\n", soundcloud_urls);
    }
}
