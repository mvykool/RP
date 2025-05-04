use actix-web::{get, App, HttpResponse, HttpServer, Responder};

//after importing methods
//use get macro
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas are available!")
}
