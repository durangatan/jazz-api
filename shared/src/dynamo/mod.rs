extern crate dynomite;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::env;

mod create;
mod query;
mod spin;

pub use create::create_album;
pub use query::{get_albums, get_by_id};
pub use spin::spin_album;

lazy_static! {
    pub static ref CLIENT: DynamoDbClient = DynamoDbClient::new(Region::UsEast1);
    pub static ref TABLE_NAME: String = env::var("ALBUMS_TABLE").unwrap_or("".to_string());
}
