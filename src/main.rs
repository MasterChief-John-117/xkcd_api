use std::thread;
use std::time::{Duration};
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
mod sqlite_chef;
mod xkcd;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn id(req: HttpRequest) -> impl Responder {
    match req.match_info().get("id") {
        Some(id_str) => {
            match id_str.parse::<i64>() {
                Ok(request_id) => {
                    if request_id > sqlite_chef::get_latest_id().into() {
                        HttpResponse::NotFound().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"id higher than existing xkcd comics\"}")
                    }
                    else {
                        match sqlite_chef::get_comic_by_id(request_id) {
                            Some(comic) => {
                                HttpResponse::Ok().header(http::header::CONTENT_TYPE, "application/json").json(comic)
                            },
                            None => {
                                HttpResponse::NotFound().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"id not found\"}")
                            }
                        }
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

async fn title(req: HttpRequest) -> impl Responder {
    match req.match_info().get("query") {
        Some(title_str) => {
            let mut matching_scs = sqlite_chef::get_search_comics();
            matching_scs.retain(|element| element.title.contains(&xkcd::normalize(&title_str)));
            let mut ids: Vec<i32> = Vec::<i32>::new();
            for scomic in matching_scs {
                ids.push(scomic.num);
            }
            HttpResponse::Ok().header(http::header::CONTENT_TYPE, "application/json").json(sqlite_chef::get_comics_by_ids(ids))
        },
        None => {
            HttpResponse::BadRequest().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"Could not parse title\"}")
        }
    }
}

async fn alt(req: HttpRequest) -> impl Responder {
    match req.match_info().get("query") {
        Some(alt_str) => {
            let mut matching_scs = sqlite_chef::get_search_comics();
            matching_scs.retain(|element| element.alt_text.contains(&xkcd::normalize(&alt_str)));
            let mut ids: Vec<i32> = Vec::<i32>::new();
            for scomic in matching_scs {
                ids.push(scomic.num);
            }
            HttpResponse::Ok().header(http::header::CONTENT_TYPE, "application/json").json(sqlite_chef::get_comics_by_ids(ids))
        },
        None => {
            HttpResponse::BadRequest().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"Could not parse alt\"}")
        }
    }
}

async fn transcript(req: HttpRequest) -> impl Responder {
    match req.match_info().get("query") {
        Some(trans_str) => {
            let mut matching_scs = sqlite_chef::get_search_comics();
            matching_scs.retain(|element| element.transcript.contains(&xkcd::normalize(&trans_str)));
            let mut ids: Vec<i32> = Vec::<i32>::new();
            for scomic in matching_scs {
                ids.push(scomic.num);
            }
            HttpResponse::Ok().header(http::header::CONTENT_TYPE, "application/json").json(sqlite_chef::get_comics_by_ids(ids))
        },
        None => {
            HttpResponse::BadRequest().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"Could not parse transcript\"}")
        }
    }
}

async fn search(req: HttpRequest) -> impl Responder {
    match req.match_info().get("query") {
        Some(query) => {
            let mut matching_scs = sqlite_chef::get_search_comics();
            matching_scs.retain(|element| element.transcript.contains(&xkcd::normalize(&query)) || 
                element.alt_text.contains(&xkcd::normalize(&query)) || 
                element.transcript.contains(&xkcd::normalize(&query)));
            let mut ids: Vec<i32> = Vec::<i32>::new();
            for scomic in matching_scs {
                ids.push(scomic.num);
            }
            HttpResponse::Ok().header(http::header::CONTENT_TYPE, "application/json").json(sqlite_chef::get_comics_by_ids(ids))
        },
        None => {
            HttpResponse::BadRequest().header(http::header::CONTENT_TYPE, "application/json").body("{\"error\": \"Could not parse query\"}")
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    sqlite_chef::ensure_tables();

    // Create database update thread
    thread::spawn(|| {
        loop {
            let latest_comic_id: i32 = xkcd::get_latest_comic().num;
            let latest_stored_comic_id: i32 = sqlite_chef::get_latest_id();

            println!("Latest Comic: {}", latest_comic_id);
            println!("Latest Stored Comic: {}", latest_stored_comic_id);

            if latest_stored_comic_id < latest_comic_id {
                for next_id in (latest_stored_comic_id+1)..(latest_comic_id+1) {
                    println!("Fetching {}", next_id);
                    let next_comic: xkcd::Comic = xkcd::get_comic_by_id(next_id);
                    println!("{:?}", next_comic);
                    sqlite_chef::insert_comic_both(next_comic);
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
            .route("/id/{id}", web::get().to(id))
            .route("/title/{query}", web::get().to(title))
            .route("/alt/{query}", web::get().to(alt))
            .route("/transcript/{query}", web::get().to(transcript))
            .route("/search/{query}", web::get().to(search))
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
