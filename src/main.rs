use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::env;

async fn request_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/healthz") => Ok(Response::new(Body::from("OK"))),
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = match env::var("PORT") {
      Ok(port_string) => port_string.parse::<u16>().unwrap(),
      Err(_) => 4000,
    };
    let addr = ([0, 0, 0, 0], port).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(request_handler)) });
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
