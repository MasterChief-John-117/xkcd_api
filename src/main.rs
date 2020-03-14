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
    sqlite_chef::ensure_tables();

    // Create database update thread
    thread::spawn(|| {
        loop {
            let latest_comic_id: i32 = xkcd::get_latest().num;
            let latest_stored_comic_id: i32 = sqlite_chef::get_latest();

            println!("Latest Comic: {}", latest_comic_id);
            println!("Latest Stored Comic: {}", latest_stored_comic_id);

            if latest_stored_comic_id < latest_comic_id {
                for next_id in (latest_stored_comic_id+1)..latest_comic_id {
                    println!("Fetching {}", next_id);
                    let next_comic: xkcd::Comic = xkcd::get_by_id(next_id);
                    println!("{:?}", next_comic);
                    sqlite_chef::insert_comic(next_comic);
                    println!("Inserted {}", next_id);
                    thread::sleep(Duration::from_secs(1)); // Wait a second so we don't hammer xkcd
                }
            }
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
