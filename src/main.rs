use reqwest::{Client, Request};
use serde::Deserialize;
use std::{
    error::Error,
    fmt::{self, write}, io::Read,
};

#[derive(Deserialize, Debug)]
enum Method {
    #[serde(alias = "get")]
    GET,
}

impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        match self {
            Self::GET => reqwest::Method::GET,
        }
    }
}

#[derive(Deserialize, Debug)]
struct RequestParams {
    method: Method,
    path: String,
}

#[derive(Deserialize, Debug)]
struct Req {
    request_name: String,
    params: RequestParams,
}

impl Into<reqwest::blocking::Request> for Req {
    fn into(self) -> reqwest::blocking::Request {
        reqwest::blocking::Request::new(self.params.method.into(), self.params.path.parse().unwrap())
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
    let request: reqwest::blocking::Request = serde_json::from_str::<Req>(
        std::fs::read_to_string("request.json")
            .map_err(|e| AppError {
                message: e.to_string(),
            })?
            .as_str(),
    )
    .map_err(|e| AppError {
        message: e.to_string(),
    })?.into();
    
    let mut response = client.execute(request).map_err(|e| AppError {
        message: e.to_string()
    })?;
    
    let mut body = String::new(); 
    response.read_to_string(&mut body).unwrap();
    println!("{:?}", body);
    
    Ok(())
}
