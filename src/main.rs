use std::thread;
use std::time::{Duration};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod sqlite_chef;
mod xkcd;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create database update thread
    thread::spawn(|| {
        loop {
            let latest_comic: xkcd::Comic = xkcd::get_latest();
            println!("Latest Comic: {}: {}", latest_comic.num, latest_comic.title);
            // After running the update, sleep for 30 seconds
            thread::sleep(Duration::from_secs(60 * 30)); // 30 minutes
        }
    });
    
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
