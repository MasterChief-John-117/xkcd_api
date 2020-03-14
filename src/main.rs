use std::thread;
use std::time::{Duration};
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::json;
mod sqlite_chef;
mod xkcd;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn id(req: HttpRequest) -> impl Responder {
    match req.match_info().get("id") {
        Some(id_str) => {
            match id_str.parse::<i32>() {
                Ok(request_id) => {
                    if request_id > sqlite_chef::get_latest() {
                        HttpResponse::NotFound().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"id higher than existing xkcd comics\"}")
                    }
                    else {
                        let mut matching = sqlite_chef::get_all();
                        matching.retain(|element| element.num == request_id);
                        HttpResponse::Ok().header(http::header::CONTENT_TYPE, "application/json").json(matching)                    
                    }
                },
                Err(_) => {
                    HttpResponse::BadRequest().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"Could not parse id\"}")
                }
            }
        },
        None => {
            HttpResponse::BadRequest().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"Could not parse id\"}")
        }
    }
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
                for next_id in (latest_stored_comic_id+1)..(latest_comic_id+1) {
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
            .route("/{id}", web::get().to(id))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
