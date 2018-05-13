#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use]
extern crate log;
extern crate nanoid;
extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;

use rocket::request::Form;
use rocket::State;
use rusqlite::{Connection, DatabaseName, Error};
use std::sync::Mutex;

type DbConn = Mutex<Connection>;

static DB_PATH: &'static str = "./db.sqlite";

fn init_database(conn: &mut Connection) {
    conn.restore(DatabaseName::Main, DB_PATH, None)
        .expect("Could not load the database from the file.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts (
            id              TEXT PRIMARY KEY,
            content         TEXT NOT NULL
        )",
        &[],
    ).expect("Could not create the posts table");
}

#[get("/api/<id>")]
fn post(id: String, db_conn: State<DbConn>) -> Result<String, Error> {
    db_conn.lock().expect("db connection lock").query_row(
        "SELECT content FROM posts WHERE id = ?1",
        &[&id],
        |row| row.get(0),
    )
}

#[derive(FromForm)]
struct NewPost {
    content: String,
}

#[post("/api/new", data = "<post_form>")]
fn new(post_form: Form<NewPost>, db_conn: State<DbConn>) -> Result<String, Error> {
    let post: NewPost = post_form.into_inner();
    let id = nanoid::generate(5);
    let db = db_conn.lock().expect("db connection lock");
    let result = db.execute(
        "INSERT INTO posts (id, content)
                  VALUES (?1, ?2)",
        &[&id, &post.content],
    );
    match result {
        Ok(_) => {
            if let Err(err) = db.backup(DatabaseName::Main, "./db.sqlite", None) {
                error!("{}", err);
            }
            Ok(id)
        }
        Err(err) => Err(err),
    }
}

fn main() {
    let mut conn = Connection::open_in_memory().unwrap();

    init_database(&mut conn);

    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![new, post])
        .launch();
}
