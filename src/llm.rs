use anyhow::{anyhow, Result};
use reqwest::{header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}, Client, Response};
use serde::{Deserialize, Serialize};
use async_trait;
// use async_trait::async_trait;
// use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageData,
}

#[derive(Deserialize, Debug)]
struct MessageData {
    content: String,
}

#[async_trait::async_trait]
pub trait LLMClient {
    async fn send_message(&self, api_key: &str, model: &str, content: &str) -> Result<String>;
}

pub struct OpenAIClient {
    api_url: String,
}

impl OpenAIClient {
    pub fn new(api_url: &str) -> Self {
        OpenAIClient {
            api_url: api_url.to_string(),
        }
    }
}

// why here ?
#[async_trait::async_trait]
impl LLMClient for OpenAIClient {
    async fn send_message(&self, api_key: &str, model: &str, content: &str) -> Result<String> {
        let client = Client::new();
        let url = &self.api_url;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let request_body = ChatCompletionRequest {
            model,
            messages: vec![Message {
                role: "user",
                content,
            }],
        };

        let response = client
            .post(url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_response: ChatCompletionResponse = response.json().await?;
            if let Some(choice) = chat_response.choices.into_iter().next() {
                 Ok(choice.message.content)
            } else {
                Err(anyhow!("No content in response from LLM"))
            }
        } else {
            let error_body = response.text().await?;
            Err(anyhow!("LLM API return an error: {}", error_body))
        }
    }
}

pub async fn send_to_llm(client: &dyn LLMClient, api_key: &str, model: &str, content: &str) -> Result<String> {
    client.send_message(api_key, model, content).await
}
