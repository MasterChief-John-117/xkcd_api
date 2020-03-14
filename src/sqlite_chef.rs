use std::convert::TryInto;
use super::xkcd;
use sqlite::State;

pub fn get_latest()  -> i32 {
    let connection = sqlite::open("./test.db").unwrap();
     let mut statement = match connection.prepare("SELECT MAX(num) FROM comics WHERE num > ?") {
        Ok(obj) => obj,
        Err(_) => {
            return 0;
        }
    };
    
    match statement.bind(1, 0) {
        Ok(_) => (),
        Err(_) => {
            return 0;
        }
    }
    
    while let State::Row = statement.next().unwrap() {
        return statement.read::<i64>(0).unwrap().try_into().unwrap();
    }
    return 0;
}

pub fn insert_comic(comic: xkcd::Comic) {
    let connection = sqlite::open("./test.db").unwrap();
    let _res = connection.execute(
        format!("INSERT INTO comics VALUES ({num}, {title}, {alt_text}, {transcript}, {img}, {year}, {month}, {day})", 
        num=comic.num, title=comic.title, alt_text=comic.alt_text, transcript=comic.alt_text, img=comic.img, year=comic.year, month=comic.month, day=comic.day)
    );
}

pub fn ensure_tables() {
    let connection = sqlite::open("./test.db").unwrap();

    let _res = connection.execute(
        "CREATE TABLE comics (
                  num             INTEGER PRIMARY KEY,
                  title           TEXT,
                  alt_text        TEXT,
                  transcript      TEXT,
                  img             TEXT,
                  year            INTEGER,
                  month           INTEGER,
                  day             INTEGERL
                  )"
    );
}