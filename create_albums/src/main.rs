#[macro_use]
extern crate lambda_runtime as lambda;

#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;
use std::error::Error;

use shared::{dynamo, Album, Headers, HttpPostEvent, HttpResponse};

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: HttpPostEvent, _c: lambda::Context) -> Result<HttpResponse, HandlerError> {
    println!("Creating albums from event: {:?}", e);
    let albums: Vec<Album> = serde_json::from_str(&e.body).unwrap_or(Vec::new());
    for album in albums {
        create_album(album);
    }
    Ok(HttpResponse {
        status_code: 200,
        body: "".to_string(),
        headers: Headers {
            access_control_allow_origin: "*".to_string(),
            access_control_allow_credentials: true,
        },
    })
}

fn create_album(album: Album) {
    let result = dynamo::create_album(&album);
    match result {
        Ok(_r) => println!("Album created"),
        Err(e) => {
            println!("error creating Album, {:?}", e);
        }
    };
}
