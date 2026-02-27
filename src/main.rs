use reqwest::{
    blocking::{Request, RequestBuilder},
    header::HeaderMap,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self},
    io::Read,
};

#[derive(Deserialize, Debug)]
enum Method {
    #[serde(alias = "get")]
    GET,
    #[serde(alias = "post")]
    POST,
    #[serde(alias = "put")]
    PUT,
    #[serde(alias = "patch")]
    PATCH,
    #[serde(alias = "delete")]
    DELETE,
}

impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        match self {
            Self::GET => reqwest::Method::GET,
            Self::POST => reqwest::Method::POST,
            Self::PUT => reqwest::Method::PUT,
            Self::PATCH => reqwest::Method::PATCH,
            Self::DELETE => reqwest::Method::DELETE
        }
    }
}

#[derive(Deserialize, Debug)]
struct Req {
    method: Method,
    path: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
}

impl Req {
    fn into_req(self, client: reqwest::blocking::Client) -> reqwest::blocking::RequestBuilder {
        let mut request = RequestBuilder::from_parts(
            client,
            Request::new(self.method.into(), self.path.parse().unwrap()),
        );

        if let Some(h) = self.headers {
            let mut headers = HeaderMap::new();
            h.into_iter().for_each(|(key, value)| {
                headers.append(key.leak() as &str, value.parse().unwrap());
            });

            request = request.headers(headers);
        }

        if let Some(b) = self.body {
            request = request.json(&b);
        }

        request
    }
}

#[derive(Debug)]
struct AppError {
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

impl Error for AppError {}

fn main() -> Result<(), AppError> {
    let client = reqwest::blocking::Client::new();
    let request: Req = serde_json::from_str::<Req>(
        std::fs::read_to_string("request.json")
            .map_err(|e| AppError {
                message: e.to_string(),
            })?
            .as_str(),
    )
    .map_err(|e| AppError {
        message: e.to_string(),
    })?;

    let mut response = request.into_req(client).send().map_err(|e| AppError {
        message: e.to_string(),
    })?;

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    println!("{body}");
    Ok(())
}
