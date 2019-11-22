use chrono::prelude::Utc;
use dynomite::Item;
use std::fmt;

use uuid::Uuid;
fn create_id() -> String {
    Uuid::new_v4().to_string()
}

fn timestamp_string() -> String {
    Utc::now().to_string()
}

fn no_spins() -> i32 {
    0
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug, Item, PartialEq)]
pub struct Album {
    #[serde(default = "create_id")]
    #[dynomite(partition_key)]
    pub id: String,
    title: String,
    artist: String,
    year: String,
    #[serde(default = "no_spins")]
    pub spins: i32,
    #[serde(default = "timestamp_string", rename = "dateAdded")]
    date_added: String,
    #[serde(default = "timestamp_string", rename = "dateUpdated")]
    date_updated: String,
}

impl Album {
    pub fn spin(&mut self) {
        self.spins = self.spins + 1;
    }
}

impl fmt::Display for Album {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Into<String> for Album {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_album_to_string() {
        let album = Album {
            id: "foo".into(),
            title: "Kind Of Blue".into(),
            artist: "Miles Davis".into(),
            year: "1959".into(),
            date_added: "now".into(),
            date_updated: "now".into(),
            spins: 0,
        };
        let desired_string = "{\"id\":\"foo\",\"title\":\"Kind Of Blue\",\"artist\":\"Miles Davis\",\"year\":\"1959\",\"spins\":0,\"dateAdded\":\"now\",\"dateUpdated\":\"now\"}";
        assert_eq!(
            serde_json::to_string(&album).unwrap(),
            desired_string.to_string()
        )
    }
    #[test]
    fn test_album_from_string() {
        let string = "{\"id\":\"foo\",\"title\":\"Kind Of Blue\",\"artist\":\"Miles Davis\",\"year\":\"1959\",\"spins\":0,\"dateAdded\":\"now\",\"dateUpdated\":\"now\"}";
        let desired_album = Album {
            id: "foo".into(),
            title: "Kind Of Blue".into(),
            artist: "Miles Davis".into(),
            year: "1959".into(),
            date_added: "now".into(),
            date_updated: "now".into(),
            spins: 0,
        };
        assert_eq!(
            serde_json::from_str::<Album>(&string.to_string()).unwrap(),
            desired_album
        )
    }
}
