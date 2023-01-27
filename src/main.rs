use actix_web::{get, App, HttpServer, Responder};

static HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[get("/get-soundcloud-urls")]
async fn greet() -> impl Responder {
    let urls = get_soundcloud_urls();
    format!("{:?}", urls)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind((HOST, PORT))?
        .run()
        .await
}

fn get_soundcloud_urls() -> Vec<String> {
    let soundcloud_urls = vec![String::from("hi"), String::from("goodbye")];
    return soundcloud_urls;
}
