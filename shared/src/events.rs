use crate::Album;
use lambda::error::HandlerError;

#[derive(Deserialize, Clone, Debug)]
pub struct HttpPostEvent {
    pub body: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct HttpResponse {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub body: String,
    pub headers: Headers,
}

#[derive(Serialize, Clone, Debug)]
pub struct Headers {
    #[serde(rename = "Access-Control-Allow-Origin")]
    pub access_control_allow_origin: String,
    #[serde(rename = "Access-Control-Allow-Credentials")]
    pub access_control_allow_credentials: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HttpGetEvent {}

pub fn http_response(body: &String) -> Result<HttpResponse, HandlerError> {
    let response = HttpResponse {
        status_code: 200,
        body: body.to_string(),
        headers: Headers {
            access_control_allow_origin: "*".to_string(),
            access_control_allow_credentials: true,
        },
    };
    println!("responding with: {:?}", response);
    Ok(response)
}
pub fn respond_with_albums(albums: Vec<Album>) -> Result<HttpResponse, HandlerError> {
    http_response(&serde_json::to_string(&albums).unwrap_or("[]".to_string()))
}
