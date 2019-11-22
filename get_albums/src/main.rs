#[macro_use]
extern crate lambda_runtime as lambda;

use lambda::error::HandlerError;

use std::error::Error;

use shared::{dynamo, Album, Headers, HttpGetEvent, HttpResponse};
fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: HttpGetEvent, _c: lambda::Context) -> Result<HttpResponse, HandlerError> {
    println!("Get Albums Event: {:?}", e);
    return respond_with_albums(dynamo::get_albums());
}

fn respond_with_albums(records: Vec<Album>) -> Result<HttpResponse, HandlerError> {
    let response = HttpResponse {
        status_code: 200,
        body: serde_json::to_string(&records).unwrap_or("[]".to_string()),
        headers: Headers {
            access_control_allow_origin: "*".to_string(),
            access_control_allow_credentials: true,
        },
    };
    println!("GET response:{:?}", response);
    Ok(response)
}
