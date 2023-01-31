use rusqlite::{Connection, Result};

#[derive(Debug)]
struct SoundcloudUrl {
    id: i32,
    url: String,
}

pub fn insert_soundcloud_url(url: &String) -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE soundcloudurl (
            id    INTEGER PRIMARY KEY,
            url   TEXT NOT NULL,
        )",
        (), // empty list of parameters.
    )?;
    let soundcloud_url = SoundcloudUrl { id: 0, url: *url };
    conn.execute(
        "INSERT INTO soundcloudurl (url) VALUES (?1)",
        (&soundcloud_url.url),
    )?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_soundcloud_url() {
        let url = String::from("https://soundcloud.com/thisismyurl");
        assert_eq!(insert_soundcloud_url(&url), Ok(()));
    }
}
