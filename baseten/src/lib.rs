
use std::{collections::HashMap, error::Error};
//use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, ACCEPT};
//use serde_json::{json, to_string_pretty, Value};
//use reqwest::{Client, Response};

const BASE_URI: &str = "https://app.baseten.co";
//const BASE_URI: &str = "http://localhost:8000";

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

 pub async fn call_model_prompt(model_id:&String, prompt:&String, baseten_api_key:&String) -> Result<String, Box<dyn Error>> {
    let uri = format!("{}/models/{}/predict", BASE_URI.to_string(), model_id.to_string());
    let client = reqwest::Client::builder().build()?;
    let mut post_body = HashMap::new();
    post_body.insert("prompt", prompt);
    let body = client
    .post(uri)
    .json(&post_body)
    .header("Authorization", format!("Api-Key {}", &baseten_api_key))
    .send().await;
    let response = body.expect("Failed to execute request.");
    let body = response.text().await;
    Ok(body.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
