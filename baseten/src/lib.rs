
///#![allow(dead_code)]

use std::{collections::HashMap, error::Error};
/// This is going to change soon, w/ model id in the subdomain of the UI.
const BASE_URI: &str = "https://app.baseten.co";

/// A struct representing a Baseten API client
#[derive()]
pub struct Baseten {
    pub api_key: String,
}

impl Baseten {
    pub fn new(api_key: String) -> Baseten {
        Baseten { api_key }
    }

    /// Returns the API key
    /// 
    /// # Arguments
    /// 
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    /// Returns a Result with the response body as a String
    /// 
    /// # Arguments
    /// 
    /// * `model_id` - The ID of the model to call
    /// * `prompt` - The prompt to send to the model
    /// * `opt_args` - An optional HashMap of arguments to send to the model
    /// 
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

    /// Returns the result of calling wake on Baseten, with Result with the response body as a String
    /// 
    /// # Arguments
    /// 
    /// * `deployment_id` - The ID of the deployment to wake
    /// 
    pub async fn wake(&self, deployment_id:&String) -> Result<String, Box<dyn Error>> {
        let uri: String = format!("{}/model_versions/{}/wake", BASE_URI.to_string(), deployment_id.to_string());
        let client = reqwest::Client::builder().build()?;
        let body = client
        .post(uri)
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
        let deployment_id = std::env::var("BASETEN_DEPLOYMENT_ID").expect("Expected environment variable BASETEN_DEPLOYMENT_ID");

        let baseten = Baseten {
            api_key: api_key.to_string()
        };
        let r: Result<String, Box<dyn Error>> = block_on(baseten.wake(&deployment_id));
        let rv = r.expect("couldnt get Result from wake()");

        assert_eq!(rv, "{}"); 
    }

    #[test]
    fn test_call_model() {
        let api_key = std::env::var("BASETEN_AUTH_TOKEN").expect("Expected environment variable BASETEN_AUTH_TOKEN");
        let model = std::env::var("BASETEN_MODEL_ID").expect("Expected environment variable BASETEN_MODEL_ID");
        println!("Model: {}", model);
        println!("API Key: {}", api_key);
        let baseten = Baseten {
            api_key: api_key.to_string()
        };
        let prompt = String::from("A tree in a field under the night sky");
        let mut opts = HashMap::new();
        opts.insert(String::from("use_refiner"), String::from("true"));

        let r: Result<String, Box<dyn Error>> = block_on(baseten.call_model_prompt(&model, &prompt, Some(opts)));
        println!("Result: {:?}", r);
        assert!(r.unwrap().len() > 10000);
    }
}
