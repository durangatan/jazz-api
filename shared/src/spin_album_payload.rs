use std::fmt;

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct SpinAlbumPayload {
    pub album_id: String,
}

impl fmt::Display for SpinAlbumPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl Into<String> for SpinAlbumPayload {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
