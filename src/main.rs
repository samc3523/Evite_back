#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;



#[derive(Serialize, Deserialize)]
struct Guest {
    id: i32,
    name: String,
    email: String,
    phone:String,
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
            name: "Samuel test".to_string(),
            email: "fake@email.com".to_string(),
            phone:"508-269-3523".to_string(),
            coming: true,

        }
    )
}


#[get("/<id>")]
fn get_guest(id: i32) -> Json<Guest> {
    Json(
      Guest {
        id,
        name: "Samuel 2test".to_string(),
        email: "fake2@fake.com".to_string(),
        phone:"508-269-3523".to_string(),
        coming: true,

      }
    )
}

#[get("/")]
fn get_all_guests() -> Json<Vec<Guest>> {
    Json(vec![
        Guest {
            id: 0,
            name: "Samuel 0test".to_string(),
            email: "fake2@fake.com".to_string(),
            phone:"508-269-3523".to_string(),
            coming: true,
        },
        Guest {
            id: 1,
            name: "Samuel 3test".to_string(),
            email: "fake2@fake.com".to_string(),
            phone:"508-269-3523".to_string(),
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
      .mount("/", routes![index])
      .mount("/guests", routes![get_all_guests,get_random_guest,get_guest,new_guest])

}




//for when the time comes to treat input 
//let x: u32 = 10;
//let s: String = x.to_string();
//println!("{}", s);

