#[macro_use]
extern crate lambda_runtime as lambda;

use lambda::error::HandlerError;

use std::error::Error;
use std::thread;

use shared::{dynamo, Headers, HttpPostEvent, HttpResponse, SpinAlbumPayload};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: HttpPostEvent, _c: lambda::Context) -> Result<HttpResponse, HandlerError> {
    println!("Spin albums event: {:?}", e);
    let payloads: Vec<SpinAlbumPayload> = serde_json::from_str(&e.body).unwrap_or(Vec::new());

    let mut handles = Vec::new();
    for payload in payloads {
        let handle = thread::spawn(move || handle_payload(&payload));
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap();
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

fn handle_payload(payload: &SpinAlbumPayload) {
    let albums = dynamo::get_by_id(&payload.album_id);
    for mut album in albums {
        album.spin();
        let result = dynamo::spin_album(&album);
        match result {
            Ok(r) => {
                println!("Spun album, {:?}", r);
            }
            Err(e) => {
                println!("error spinning album, {:?}", e);
            }
        };
    }
}
