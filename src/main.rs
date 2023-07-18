#[macro_use] extern crate rocket;
use diesel::{table, Insertable, Queryable};
use rocket::{fairing::AdHoc, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel};

#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}


#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!(
      "Hello, {}! You are {} years old.", config.name, config.age
   )
}


table! {
    guests (id) {
        id -> Int4,
        gname -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        msg -> Text,
        coming -> Bool,
    }
}

#[database("my_db")]
pub struct Db(diesel::PgConnection);
#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[diesel(table_name = guests)]
struct Guest {
    id: i32,
    gname: String,
    email: String,
    phone: String,
    msg: String,
    coming: bool,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/random")]
fn get_random_guest() -> Json<Guest> {
    Json(
        Guest {
            id: 1,
            gname: "Samuel test".to_string(),
            email: "fake@email.com".to_string(),
            phone:"508-269-3523".to_string(),
            msg: "fiouaergvoaerog".to_string(),
            coming: true,

        }
    )
}


#[get("/<id>")]
fn get_guest(id: i32) -> Json<Guest> {
    Json(
      Guest {
        id,
        gname: "Samuel 2test".to_string(),
        email: "fake2@fake.com".to_string(),
        phone:"508-269-3523".to_string(),
        msg: "fiouaergvoaerog".to_string(),
        coming: true,

      }
    )
}

#[get("/")]
fn get_all_guests() -> Json<Vec<Guest>> {
    Json(vec![
        Guest {
            id: 0,
            gname: "Samuel 0test".to_string(),
            email: "fake2@fake.com".to_string(),
            phone:"508-269-3523".to_string(),
            msg: "fiouaergvoaerog".to_string(),
            coming: true,
        },
        Guest {
            id: 1,
            gname: "Samuel 3test".to_string(),
            email: "fake2@fake.com".to_string(),
            phone:"508-269-3523".to_string(),
            msg: "fiouaergvoaerog".to_string(),
            coming: true,

        }
      ]
    )
}


#[post("/", data = "<guest>")]
fn new_guest(guest: Json<Guest>) -> Json<Guest> {
    guest
}



#[launch]
fn rocket() -> _ {
    let rocket= rocket::build();
    
    rocket
      .attach(Db::fairing())
      .attach(AdHoc::config::<Config>())
      .mount("/", routes![index,get_config])
      .mount("/guests", routes![get_all_guests,get_random_guest,get_guest,new_guest])

}




//for when the time comes to treat input 
//let x: u32 = 10;
//let s: String = x.to_string();
//println!("{}", s);

