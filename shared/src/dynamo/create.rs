use crate::{Album,dynamo::{CLIENT, TABLE_NAME}};
use dynomite::dynamodb::{DynamoDb, PutItemError, PutItemInput};
use rusoto_core::RusotoError;

pub fn create_album(album: &Album) -> Result<(), RusotoError<PutItemError>> {
    let put_input = PutItemInput {
        item: album.clone().into(),
        table_name: TABLE_NAME.clone(),
        ..PutItemInput::default()
    };
    println!("put item input, {:?}", put_input);
    let result = CLIENT.put_item(put_input).sync();
    println!("put item result, {:?}", result);

    return match result {
        Ok(_ok) => return Ok(()),
        Err(_e) => Err(_e),
    };
}
