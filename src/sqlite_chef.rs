use std::convert::TryInto;
use super::xkcd;
use sqlite::State;

pub fn get_latest_id() -> i32 {
    let connection = sqlite::open("./xkcd.db").unwrap();
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

pub fn get_comic_by_id(num : i64) -> Option<xkcd::Comic> {
    let connection = sqlite::open("./xkcd.db").unwrap();
    let mut statement = match connection.prepare("SELECT * FROM comics WHERE num = ?") {
        Ok(obj) => obj,
        Err(_) => {
            return None;
        }
    };
    
    match statement.bind(1, num) {
        Ok(_) => (),
        Err(_) => {
            return None;
        }
    }
    
    while let State::Row = statement.next().unwrap() {
        return Some(xkcd::Comic {
            num: statement.read::<i64>(0).unwrap().try_into().unwrap(),
            title: statement.read(1).unwrap(),
            alt_text: statement.read(2).unwrap(),
            transcript: statement.read(3).unwrap(),
            img: statement.read(4).unwrap(),
            year: statement.read::<i64>(5).unwrap().try_into().unwrap(),
            month: statement.read::<i64>(6).unwrap().try_into().unwrap(),
            day: statement.read::<i64>(7).unwrap().try_into().unwrap(),
        })
    }
    return None;
}

pub fn get_comics_by_ids(nums : Vec<i32>) -> Vec<xkcd::Comic> {
    let mut comics: Vec<xkcd::Comic> = Vec::<xkcd::Comic>::new();
    let connection = sqlite::open("./xkcd.db").unwrap();
    let mut statement = match connection.prepare("SELECT * FROM comics WHERE num > ?") {
        Ok(obj) => obj,
        Err(errm) => {
            println!("{}", errm);
            return comics;
        }
    };

    match statement.bind(1, 0) {
        Ok(_) => (),
        Err(errm) => {
            println!("{}", errm);
            return comics;
        }
    }
    
    while let State::Row = statement.next().unwrap() {
        comics.push(xkcd::Comic {
            num: statement.read::<i64>(0).unwrap().try_into().unwrap(),
            title: statement.read(1).unwrap(),
            alt_text: statement.read(2).unwrap(),
            transcript: statement.read(3).unwrap(),
            img: statement.read(4).unwrap(),
            year: statement.read::<i64>(5).unwrap().try_into().unwrap(),
            month: statement.read::<i64>(6).unwrap().try_into().unwrap(),
            day: statement.read::<i64>(7).unwrap().try_into().unwrap(),
        })
    }
    comics.retain(|comic| nums.contains(&comic.num));
    return comics;
}

pub fn get_search_comics() -> Vec<xkcd::SearchComic> {
    let mut comics: Vec<xkcd::SearchComic> = Vec::<xkcd::SearchComic>::new();
    let connection = sqlite::open("./xkcd.db").unwrap();
    let mut statement = match connection.prepare("SELECT * FROM search_comics WHERE num > ?") {
        Ok(obj) => obj,
        Err(_) => {
            return comics;
        }
    };
    
    match statement.bind(1, 0) {
        Ok(_) => (),
        Err(_) => {
            return comics;
        }
    }
    
    while let State::Row = statement.next().unwrap() {
        comics.push(xkcd::SearchComic {
            num: statement.read::<i64>(0).unwrap().try_into().unwrap(),
            title: statement.read(1).unwrap(),
            alt_text: statement.read(2).unwrap(),
            transcript: statement.read(3).unwrap(),
        })
    }
    return comics;
}

pub fn insert_comic_both(comic: xkcd::Comic) {
    let connection = sqlite::open("./xkcd.db").unwrap();
    let _ = connection.execute(
        format!("INSERT INTO comics VALUES ({num}, '{title}', '{alt_text}', '{transcript}', '{img}', {year}, {month}, {day})", 
        num=comic.num, title=str::replace(&comic.title, "'", "''"), alt_text=str::replace(&comic.alt_text, "'", "''"), transcript=str::replace(&comic.transcript, "'", "''"), 
        img=str::replace(&comic.img, "'", "''"), year=comic.year, month=comic.month, day=comic.day)
    ).unwrap();
    let _ = connection.execute(
        format!("INSERT INTO search_comics VALUES ({num}, '{title}', '{alt_text}', '{transcript}')", 
        num=comic.num, title=xkcd::normalize(&comic.title), alt_text=xkcd::normalize(&comic.alt_text), transcript=xkcd::normalize(&comic.transcript))
    ).unwrap();
}

pub fn ensure_tables() {
    let connection = sqlite::open("./xkcd.db").unwrap();

    match connection.execute(
        "CREATE TABLE comics (
                  num             INTEGER PRIMARY KEY,
                  title           TEXT,
                  alt_text        TEXT,
                  transcript      TEXT,
                  img             TEXT,
                  year            INTEGER,
                  month           INTEGER,
                  day             INTEGER
                  )"
    ) {
        Ok(_) => {},
        Err(msg) => { println!("Error: {}", msg); }
    }
    match connection.execute(
        "CREATE TABLE search_comics (
                  num             INTEGER PRIMARY KEY,
                  title           TEXT,
                  alt_text        TEXT,
                  transcript      TEXT
                  )"
    ) {
        Ok(_) => {},
        Err(msg) => { println!("Error: {}", msg); }
    }
}