#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/posts")]
fn get_all_posts() -> &'static str {
    "Hello, world!"
}

#[get("/post")]
fn get_one_post() -> &'static str {
    "Hello, world!"
}

#[post("/new")]
fn create_new_post() -> &'static str {
    "Hello, world!"
}
#[put("/update")]
fn update_one_post() -> &'static str {
    "Hello, world!"
}

#[post("/publish")]
fn publish_post() -> &'static str {
    "Hello, world!"
}

#[delete("/delete")]
fn delete_post() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let result = rocket::build().mount("/", routes![index]).launch().await;
    match result {
        Ok(_) => println!("Server started"),
        Err(e) => println!("Error: {}", e),
    }
}
