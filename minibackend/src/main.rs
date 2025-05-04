use actix_web::{get, post, patch, App, HttpResponse, HttpServer, Responder};

//after importing methods
//use get macro
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas are available!")
}

//adding post and patch macros/endpoints
#[post("/buy")]
async fn buy_pizza() -> impl Responder {
    HttpResponse::Ok().body("You just bought a pizza")
}

#[patch("/update/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("pizza was updated")
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
