#[macro_use]
extern crate lambda_runtime as lambda;

use lambda::error::HandlerError;

use std::error::Error;
use std::thread;

use shared::{dynamo, HttpPostEvent, SpinAlbumPayload};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: HttpPostEvent>, _c: lambda::Context) -> Result<(), HandlerError> {
    println!("Spin albums event: {:?}", e);
    for payload in e.body {
        spin_album(&payload);
    }
    Ok(())
}

fn spin_album(payload: &SpinAlbumPayload) {
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
