#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate lambda_runtime as lambda;
extern crate rand;

mod spin_album_payload;
mod album;
pub mod dynamo;
pub mod events;
pub use album::Album;
pub use events::{http_response, Headers, HttpGetEvent, HttpPostEvent, HttpResponse};
pub use spin_album_payload::SpinAlbumPayload;
