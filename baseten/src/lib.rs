
#![allow(dead_code)]

use std::{collections::HashMap, error::Error};
const BASE_URI: &str = "https://app.baseten.co";


#[derive()]
pub struct Baseten {
    pub api_key: String,
}

impl Baseten {
    pub fn new(api_key: String) -> Baseten {
        Baseten { api_key }
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    pub async fn call_model_prompt(&self, model_id:&String, prompt:&String, opt_args:Option<HashMap<String, String>>) -> Result<String, Box<dyn Error>> {
        let uri = format!("{}/models/{}/predict", BASE_URI.to_string(), model_id.to_string());
        let client = reqwest::Client::builder().build()?;
        let mut post_body = HashMap::new();

        match opt_args {
            Some(args) => {
                for (key, value) in args {
                    post_body.insert(key.to_string(), value);
                }
            },
            None => {}
        }

        post_body.insert(String::from("prompt"), prompt.clone());
        let body = client
        .post(uri)
        .json(&post_body)
        .header("Authorization", format!("Api-Key {}", &self.api_key))
        .send().await;
        let response = body.expect("Failed to execute request.");
        let body = response.text().await;
        Ok(body.unwrap())
    }

    pub async fn wake(&self, model_id:&String) -> Result<String, Box<dyn Error>> {
        let uri = format!("{}/model_versions/{}/wake", BASE_URI.to_string(), model_id.to_string());
        let client = reqwest::Client::builder().build()?;
        let body = client
        .post(uri)
        //.json(&post_body)
        .header("Authorization", format!("Api-Key {}", &self.api_key))
        .send().await;
        let response = body.expect("Failed to execute request.");
        let body = response.text().await;        
        Ok(body.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test; 
    use tokio_test::block_on;

    #[test]
     fn test_wake() {

        let api_key = std::env::var("BASETEN_AUTH_TOKEN").expect("Expected environment variable BASETEN_AUTH_TOKEN");
        let model = std::env::var("BASETEN_MODEL_ID").expect("Expected environment variable BASETEN_MODEL_ID");

        let baseten = Baseten {
            api_key: api_key.to_string()
        };
        let r: Result<String, Box<dyn Error>> = block_on(baseten.wake(&model));
        match r {
            Ok(s) => println!("Returned: {}", s),
            Err(e) => println!("Error: {}", e),
        }
        assert_eq!(4, 4);
    }

    #[test]
    fn test_call_model() {
        assert_eq!(4,4);
    }
}
