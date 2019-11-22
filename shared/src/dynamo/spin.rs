use crate::{
    dynamo::{CLIENT, TABLE_NAME},
    Album,
};

use chrono::prelude::Utc;
use dynomite::{
    attr_map,
    dynamodb::{DynamoDb, UpdateItemError, UpdateItemInput},
    FromAttributes,
};
use rusoto_core::RusotoError;

fn get_spin_input(album: &Album) -> UpdateItemInput {
    let update_expression = vec!["#a = :a", "#b = :b"];
    let expression_attribute_names = hashmap! {
        String::from("#a") => String::from("spins"),
        String::from("#b") => String::from("dateUpdated"),
    };

    let expression_attribute_values = attr_map! {
        ":a" => album.spins,
        ":b" => Utc::now().to_string(),
    };

    UpdateItemInput {
        table_name: TABLE_NAME.clone(),
        key: attr_map! {
        "id" => album.id.clone(),
        },
        update_expression: Some(format!("SET {}", update_expression.join(","))),
        expression_attribute_names: Some(expression_attribute_names),
        expression_attribute_values: Some(expression_attribute_values),
        return_values: Some("ALL_NEW".to_string()),
        ..UpdateItemInput::default()
    }
}

pub fn spin_album(album: &Album) -> Result<Album, RusotoError<UpdateItemError>> {
    let spin_input = get_spin_input(album);
    println!("Input to dynamo spin update: {:?}", spin_input);
    let result = CLIENT.update_item(spin_input).sync();
    return match result {
        Ok(result) => {
            return match result.attributes {
                Some(attrs) => Ok(Album::from_attrs(attrs).unwrap()),
                None => Err(RusotoError::Service(UpdateItemError::InternalServerError(
                    "error spinning album".to_string(),
                ))),
            };
        }
        Err(_e) => Err(_e),
    };
}
