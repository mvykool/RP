use actix_web::{web::Path, get, post, patch, web::Json, App, HttpResponse, HttpServer, Responder};
mod models;
use validator::Validate;
use crate::models::pizza::{BuyRequest, UpdatePizza};

//use get macro
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas are available!")
}

//adding post and patch macros/endpoints
#[post("/buy")]
async fn buy_pizza(body: Json<BuyRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("pizza entered is: {pizza_name}"))
        },
        Err(_) => HttpResponse::Ok().body("please enter pizza")
    }
}

#[patch("/update/{uuid}")]
async fn update_pizza(update_pizza: Path<UpdatePizza>) -> impl Responder {
    let uuid = update_pizza.into_inner().uuid;
    HttpResponse::Ok().body(format!("pizza was updated: {uuid}"))
}

//main macro, creating entry to the program
#[actix_web::main]
//create asyncrounous function named main, with return type result, and use httpserver
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
        })

        .bind("127.0.0.1:8080")?
        .run()
        .await
}
