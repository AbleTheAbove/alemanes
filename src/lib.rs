use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/{id}/{name}")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// Step 1: We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
// Step 2: Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(index).service(health_check))
        .bind("127.0.0.1:8000")?
        .run();
    // Step 2 (as above):
    // .await
    // In run we invoke (and await) HttpServer::run. HttpServer::run returns an instance of Server - when we call .await it starts listening on the address we specified indefinitely: it will handle incoming requests as they arrive, but it will never shutdown or "complete" on its own.
    // We need to run our application as a background task.
    // tokio::spawn comes quite handy here: tokio::spawn takes a future and hands it over to the runtime for polling, without waiting for its completion; it therefore runs concurrently with downstream futures and tasks (e.g. our test logic).

    Ok(server)
}
