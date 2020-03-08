extern crate serde;
extern crate serde_derive;
use serde_derive::*;
extern crate mio_httpc;
use mio_httpc::CallBuilder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Comic {
    pub num: i32,
    pub title: String,
    pub alt_text: String,
    pub transcript: String,
    pub year: i32,
    pub month: i32,
    pub day: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct ApiComic {
    pub num: i32,
    pub safe_title: String,
    pub alt: String,
    pub transcript: String,
    pub year: String,
    pub month: String,
    pub day: String,
}

pub fn get_latest() -> Comic {
    let (_response_meta, body) = CallBuilder::get().timeout_ms(5000).url("https://xkcd.com/info.0.json").unwrap().exec().unwrap();

    let latest_comic: ApiComic = serde_json::from_str(&String::from_utf8(body).unwrap()).unwrap();
    
    return Comic {
        num: latest_comic.num,
        title: latest_comic.safe_title,
        alt_text: latest_comic.alt,
        transcript: latest_comic.transcript,
        year: latest_comic.year.parse::<i32>().unwrap(),
        month: latest_comic.month.parse::<i32>().unwrap(),
        day: latest_comic.day.parse::<i32>().unwrap(),
    }
}