// #[macro_use] extern crate rocket;

// #[get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![hello])
// }

use std::collections::HashMap;

use rocket::futures::lock::Mutex;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{catch, catchers, get, post, put, routes, State};

type PepoleItems = Mutex<HashMap<usize, Pepole>>;
type Messages<'r> = &'r State<PepoleItems>;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Pepole {
    id: usize,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: usize,
    name: String,
}

#[get("/pepole/<id>")]
async fn get_pepeole(id: usize, message: Messages<'_>) -> Json<Pepole> {
    let pepole_map = message.lock().await;
    if id == 0 {
        return Json(Pepole {
            id: 0,
            name: "_".to_string(),
        });
    }
    match pepole_map.get(&id) {
        None => Json(Pepole {
            id: 0,
            name: "_1".to_string(),
        }),
        Some(p) => Json(p.to_owned()),
    }
}

#[get("/pepole", format = "json", data = "<pepole>")]
async fn create_pepeole(pepole: Json<Pepole>, message: Messages<'_>) -> Value {
    let mut pepole_map = message.lock().await;
    let new_person = pepole.into_inner();
    if pepole_map.contains_key(&new_person.id) {
        json!({"res":"err"})
    } else {
        pepole_map.insert(new_person.id, new_person);
        json!({"res":"ok"})
    }
}

#[get("/")]
async fn hello() -> Option<Json<Task>> {
    Some(Json(Task {
        id: 1,
        name: "Tom".to_string(),
    }))
}

#[get("/exs/<id>")]
async fn get_exs(id: usize) -> Value {
    json!({"res":"ex"})
}

#[post("/ex", format = "json", data = "<task>")]
async fn post_ex(task: Json<Task>) -> Value {
    let task = task.into_inner();
    json!({ "res": format!("{} {}", task.id, task.name) })
}

#[put("/ex/<id>")]
async fn put_ex(id: usize) -> String {
    "ex_put".to_string()
}

#[catch(404)]
async fn not_rust() -> Value {
    json!("404")
}

#[catch(404)]
async fn not_base() -> Value {
    json!("base 404")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .manage(PepoleItems::new(HashMap::new()))
        // route
        .mount("/hello", routes![hello])
        .mount("/base", routes![get_exs, post_ex, put_ex])
        // catch
        .register("/", catchers!(not_rust))
        .register("/base", catchers!(not_base))
        .launch()
        .await?;
    Ok(())
}
