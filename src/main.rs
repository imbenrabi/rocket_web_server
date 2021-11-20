#![feature(proc_macro_hygiene, decl_macro)]

use rusqlite::Connection;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello from Rust!"
}

fn main() {
    {
        let db_connection = Connection::open("data.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists todo_list (
                    id integer primary key,
                    item varchar(64) not null
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();
    }

    rocket::ignite().mount("/", routes![index]).launch();
}
