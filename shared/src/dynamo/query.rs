use crate::{
    dynamo::{CLIENT, TABLE_NAME},
    Album,
};
use dynomite::{
    attr_map,
    dynamodb::{DynamoDb, QueryInput, ScanInput},
    FromAttributes,
};

pub fn get_by_id(id: &String) -> Vec<Album> {
    let query_input = QueryInput {
        table_name: TABLE_NAME.clone(),
        key_condition_expression: Some("id = :id".into()),
        expression_attribute_values: Some(attr_map! {
            ":id" => id.to_string()
        }),
        consistent_read: Some(true),
        ..QueryInput::default()
    };
    return query(query_input);
}

pub fn get_albums() -> Vec<Album> {
    let result = CLIENT
        .scan(ScanInput {
            table_name: TABLE_NAME.to_string(),
            ..ScanInput::default()
        })
        .sync();
    println!("Scan result for albums table, {:?}", result);
    match result {
        Ok(scan_output) => scan_output
            .items
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|item| Album::from_attrs(item).unwrap())
            .collect(),
        Err(error) => {
            println!("encountered error: {:?}", error);
            return Vec::new();
        }
    }
}

fn query(query_input: QueryInput) -> Vec<Album> {
    println!("Query input for get by id, {:?}", query_input);
    let result = CLIENT.query(query_input).sync();
    println!("Query result for get by id, {:?}", result);
    match result {
        Ok(query_output) => query_output
            .items
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|item| Album::from_attrs(item).unwrap())
            .collect(),
        Err(error) => {
            println!("encountered error: {:?}", error);
            return Vec::new();
        }
    }
}
