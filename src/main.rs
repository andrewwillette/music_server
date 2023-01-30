use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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
    fn from(urls: Vec<String>) -> Self {
        return SoundcloudUrls { urls };
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
async fn get_soundcloud_urls() -> impl Responder {
    let urls = get_db_soundcloud_urls();
    let soundcloud_urls = SoundcloudUrls::from(urls);
    return format!("{:?}", soundcloud_urls);
    // let urls = SoundcloudUrls { urls };
    // format!("{:?}", urls)
    //
}

#[post("/soundcloud-url")]
async fn add_soundcloud_url(_json: web::Json<SoundcloudUrl>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn get_db_soundcloud_urls() -> Vec<String> {
    let soundcloud_urls = vec![String::from("hi"), String::from("goodbye")];
    return soundcloud_urls;
}
