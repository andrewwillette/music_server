use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub mod db;

static HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[derive(Serialize, Deserialize, Debug)]
struct SoundcloudUrl {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SoundcloudUrls {
    urls: Vec<String>,
}

impl SoundcloudUrls {
    // get api level soundcloud urls from db representation
    fn from(urls: Vec<db::SoundcloudUrl>) -> Self {
        let mut soundcloud_urls = Vec::new();
        for url in urls {
            soundcloud_urls.push(url.url);
        }
        SoundcloudUrls {
            urls: soundcloud_urls,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open(db::DB_PATH).unwrap();
    let mut db_context = db::DbContext::new(&conn);
    if let Err(result) = db_context.init_soundcloud_db() {
        eprintln!("Error initializing database: {result}");
    }
    HttpServer::new(|| {
        App::new()
            .service(get_soundcloud_urls)
            .service(add_soundcloud_url)
    })
    .bind((HOST, PORT))?
    .run()
    .await
}

#[get("/soundcloud-urls")]
async fn get_soundcloud_urls() -> HttpResponse {
    let conn = Connection::open(db::DB_PATH).unwrap();
    let mut db_context = db::DbContext::new(&conn);
    if let Ok(soundcloud_urls) = db_context.get_soundcloud_urls() {
        let urls = SoundcloudUrls::from(soundcloud_urls);
        HttpResponse::Ok().json(&urls)
    } else {
        HttpResponse::InternalServerError().json("")
    }
}

#[post("/soundcloud-url")]
async fn add_soundcloud_url(json: web::Json<SoundcloudUrl>) -> HttpResponse {
    let conn = Connection::open(db::DB_PATH).unwrap();
    let mut db_context = db::DbContext::new(&conn);
    if db_context.insert_soundcloud_url(&json.url).is_ok() {
        HttpResponse::Created().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
