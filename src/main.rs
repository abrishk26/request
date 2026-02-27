use serde::Deserialize;
use std::{
    error::Error,
    fmt::{self},
    io::Read,
    collections::HashMap
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
struct Req {
    method: Method,
    path: String,
    headers: Option<HashMap<String, String>> 
}


impl Into<reqwest::blocking::Request> for Req {
    fn into(self) -> reqwest::blocking::Request {
        let mut request = reqwest::blocking::Request::new(
            self.method.into(),
            self.path.parse().unwrap(),
        );
        
        self.headers.and_then(|h| {
            let headers = request.headers_mut();
            
            h.into_iter().for_each(|(key, value)| {
                headers.append(key.leak() as &str, value.parse().unwrap());
            });
            
            Some(())
        });
        
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

    let mut response = client.execute(request.into()).map_err(|e| AppError {
        message: e.to_string(),
    })?;

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    println!("{body}");
    Ok(())
}
