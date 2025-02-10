use crate::{
    model::{api_response::ApiResponse, exception::setup_error::SetupError},
    utils::helper::Helper,
};
use log::{error, info};
use reqwest::{
    header::{
        HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, REFERER,
        USER_AGENT,
    },
    Client, ClientBuilder, Proxy, StatusCode,
};
use serde_json::{json, Value};
use std::{collections::HashMap, fmt::Debug, time::Duration};

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone)]
pub struct ApiService {
    client: Client,
}

impl ApiService {
    pub fn new(proxy: Option<String>, referer: Option<&str>) -> Result<Self, SetupError> {
        let mut default_headers = HeaderMap::new();
        let user_agent = HeaderValue::from_str(&Helper::random_user_agent())
            .map_err(|e| SetupError::new(&format!("Invalid User Agent: {}", e)))?;

        default_headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        default_headers.insert(USER_AGENT, user_agent);

        if let Some(referer) = referer {
            let referer_val = HeaderValue::from_str(referer)
                .map_err(|_| SetupError::new("Invalid Referer header"))?;
            default_headers.insert(REFERER, referer_val);
        }

        let client_builder = ClientBuilder::new()
            .default_headers(default_headers)
            .timeout(Duration::from_secs(60));

        let client = match proxy {
            Some(proxy_url) => {
                if !Helper::is_valid_proxy_format(&proxy_url) {
                    return Err(SetupError::new(&format!(
                        "Invalid proxy format: {}",
                        proxy_url
                    )));
                }

                let proxy = Proxy::all(&proxy_url)
                    .map_err(|e| SetupError::new(&format!("Failed to use proxy: {}", e)))?;

                client_builder
                    .proxy(proxy)
                    .build()
                    .map_err(|e| SetupError::new(&format!("Api Client Builder Error : {}", e)))?
            }
            None => client_builder
                .build()
                .map_err(|e| SetupError::new(&format!("Api Client Builder Error: {}", e)))?,
        };

        Ok(ApiService { client })
    }

    pub async fn fetch(
        &self,
        endpoint: &str,
        method: Option<HttpMethod>,
        body: Option<Value>,
        token: Option<&str>,
        additional_headers: Option<HashMap<String, String>>,
    ) -> Result<ApiResponse, reqwest::Error> {
        let method = method.unwrap_or(HttpMethod::GET);
        info!(
            "Send API Request\nEndpoint: {}\nMethod: {:?}\nBody: {:?}",
            endpoint, method, body
        );

        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
        }

        if let Some(additional) = additional_headers {
            for (key, value) in additional {
                if let (Ok(name), Ok(val)) = (
                    HeaderName::from_bytes(key.as_bytes()),
                    HeaderValue::from_str(&value),
                ) {
                    headers.insert(name, val);
                } else {
                    error!("Invalid header: {} -> {}", key, value);
                }
            }
        }

        let request = match method {
            HttpMethod::GET => self.client.get(endpoint),
            HttpMethod::POST => self.client.post(endpoint),
            HttpMethod::PUT => self.client.put(endpoint),
            HttpMethod::DELETE => self.client.delete(endpoint),
        };

        let request = request.headers(headers);

        let request = if let Some(body) = body {
            request.body(body.to_string())
        } else {
            request
        };

        let response = match request.send().await {
            Ok(res) => res,
            Err(e) => {
                error!("Request failed: {}", e);
                return Err(e);
            }
        };

        let status = response.status();
        let status_text = status.canonical_reason().unwrap_or(status.as_str());

        info!("Received response: {}", status);

        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|val| val.to_str().ok())
            .unwrap_or("");

        let mut response_data = if content_type.contains("application/json") {
            match response.json().await {
                Ok(data) => ApiResponse::new(status, data),
                Err(e) => {
                    error!("Failed to parse JSON response: {}", e);
                    return Err(e);
                }
            }
        } else {
            match response.text().await {
                Ok(text) => ApiResponse::new(
                    status,
                    json!({
                        "message": text
                    }),
                ),
                Err(e) => {
                    error!("Failed to parse text response: {}", e);
                    return Err(e);
                }
            }
        };

        if status.is_success() {
            response_data.status = StatusCode::OK;
        } else if status == StatusCode::FORBIDDEN {
            response_data.data = json!({
                "message": status_text
            });
        };

        info!("Full response data: {:?}", response_data);
        info!("API request completed successfully");

        Ok(response_data)
    }
}
