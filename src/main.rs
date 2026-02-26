use serde::Deserialize;

#[derive(Deserialize)]
struct RequestParams {
    method: String,
    path: String,
}

#[derive(Deserialize)]
struct Request {
    request_name: RequestParams,
}

fn main() {
    println!("Hello, world!");
}
