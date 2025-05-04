use actix_web::{get, App, HttpResponse, HttpServer, Responder};

//after importing methods
//use get macro
#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas are available!")
}

//main macro, creating entry to the program
#[actix_web::main]
//create asyncrounous function named main, with return type result, and use httpserver
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_pizzas))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
