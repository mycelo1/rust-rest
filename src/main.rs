#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::Json;

struct HitCount(AtomicUsize);

#[derive(Deserialize)]
struct JsonRequest {
    operand1: i32,
    operand2: i32
}

#[derive(Serialize)]
struct JsonResponse {
    result: i32
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/count")]
fn count(state_hit_count: State<HitCount>) -> String {
    state_hit_count.0.fetch_add(1, Ordering::Relaxed);
    let hit_count = state_hit_count.0.load(Ordering::Relaxed);
    format!("Number of visits: {}", hit_count)
}

#[get("/cookie/add/<value>")]
fn cookie_add(mut cookies: Cookies, value: String) -> String {
    cookies.add(Cookie::new("value", value.clone()));
    format!("Cookie sent: '{}'", value)
}

#[get("/cookie/get")]
fn cookie_get(cookies: Cookies) -> Option<String> {
    cookies.get("value")
        .map(|value| format!("Cookie received: '{}'", value))
}

#[post("/json", format = "json", data = "<request>")]
fn json(request: Json<JsonRequest>) -> Option<Json<JsonResponse>> {
    Some(Json(JsonResponse {
        result: request.0.operand1 + request.0.operand2
    }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, count, cookie_add, cookie_get, json])
        .manage(HitCount(AtomicUsize::new(0)))
        .launch();
}
