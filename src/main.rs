#[macro_use] extern crate rocket;
use diesel::{table, Insertable, Queryable};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel};
use diesel::RunQueryDsl;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
// orm set up and mapping 
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

#[database("my_db")]  //connects to the config in Rocket.toml that points to postgres url
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

#[get("/")]   //returns hello world
fn index() -> &'static str {
    "Hello, world!"
}




#[get("/<id>")]  //returns a single guest using id 
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
async fn get_all_guests(connection: Db) -> Json<Vec<Guest>> {
    connection
        .run(|c| guests::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch guests")
}



#[post("/", data = "<new_guest>")]
async fn new_guest(
  connection: Db,
  new_guest: Json<Guest>,
) -> Json<Guest> {
  
    connection
        .run(move |c| {
            diesel::insert_into(guests::table)
                .values(&new_guest.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
  
}


#[launch]
fn rocket() -> _ {
    let rocket= rocket::build();
    
    rocket
      .attach(Db::fairing())
      .attach(CORS)
      .mount("/", routes![index])
      .mount("/guests", routes![get_all_guests,get_guest,new_guest])

}



//for when the time comes to treat input 
//let x: u32 = 10;
//let s: String = x.to_string();
//println!("{}", s);

