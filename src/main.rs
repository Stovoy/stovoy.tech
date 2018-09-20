extern crate actix;
extern crate actix_web;
extern crate ctrlc;
extern crate futures;

use actix_web::{
    App, fs, HttpRequest, HttpResponse, middleware, pred,
    Result, server
};
use actix_web::http::{Method, StatusCode};
use actix_web::middleware::session::{self, RequestSession};
use std::process::exit;

fn index(req: &HttpRequest) -> Result<fs::NamedFile> {
    let mut counter = 1;
    if let Some(count) = req.session().get::<i32>("counter")? {
        counter = count + 1;
        req.session().set("counter", counter)?;
    } else {
        req.session().set("counter", counter)?;
    }

    Ok(fs::NamedFile::open("static/index.html")?)
}

fn favicon(_: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

fn error_404(_: &HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

fn main() {
    ctrlc::set_handler(move || {
        println!("SIGTERM or SIGINT detected, exiting.");
        exit(1);
    }).expect("Error setting Ctrl-C handler");

    let sys = actix::System::new("stovoy.tech");

    let app_server = server::new(
        || App::new()
            .middleware(middleware::Logger::default())
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            .resource("/", |r| r.f(index))
            .resource("/favicon", |r| r.f(favicon))
            .handler("/static", fs::StaticFiles::new("static").unwrap())
            .default_resource(|r| {
                // Default to 404 for GET request.
                r.method(Method::GET).f(error_404);

                // If not GET, 405.
                r.route().filter(pred::Not(pred::Get())).f(
                    |_| HttpResponse::MethodNotAllowed());
            }))
        .shutdown_timeout(0);

    let http_address = "0.0.0.0:8080";

    println!("Starting http server on {}", http_address);
    app_server
        .bind(http_address)
        .expect(format!("Can not bind {}", http_address).as_str())
        .start();

    sys.run();
}
