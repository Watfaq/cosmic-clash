use hyper::{Request, body::Bytes};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::{BodyExt, Full};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(unix)]
use hyperlocal::{UnixConnector, Uri as UnixUri};

/// Clash HTTP API client using Unix domain socket or HTTP
#[derive(Clone)]
pub struct ClashController {
    #[cfg(unix)]
    socket_path: Option<String>,
    http_url: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Rule,
    Global,
    Direct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    /// Proxy name
    pub name: String,
    /// Proxy type (e.g., Selector, URLTest, Fallback, Direct, Reject, etc.)
    #[serde(rename = "type")]
    pub proxy_type: String,
    /// All proxy node names contained in the proxy group (only for proxy groups)
    #[serde(default)]
    pub all: Vec<String>,
    /// Currently selected proxy node name (only for proxy groups)
    pub now: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayResponse {
    pub delay: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub upload: i64,
    pub download: i64,
    pub chains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionsResponse {
    #[serde(rename = "downloadTotal")]
    pub download_total: i64,
    #[serde(rename = "uploadTotal")]
    pub upload_total: i64,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub mode: Option<Mode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProxiesResponse {
    pub proxies: HashMap<String, Proxy>,
}

impl ClashController {
    /// Create a new controller that connects via Unix domain socket
    #[cfg(unix)]
    pub fn new_unix(socket_path: String) -> Self {
        Self {
            socket_path: Some(socket_path),
            http_url: None,
        }
    }

    /// Create a new controller that connects via HTTP
    pub fn new_http(url: String) -> Self {
        Self {
            #[cfg(unix)]
            socket_path: None,
            http_url: Some(url),
        }
    }

    /// Get all proxies
    pub async fn get_proxies(&self) -> Result<Vec<Proxy>, String> {
        let response: ProxiesResponse = self.request("GET", "/proxies", None).await
            .map_err(|e| format!("Failed to get proxies: {}", e))?;
        Ok(response.proxies.values().cloned().collect())
    }

    /// Select a proxy for a group
    pub async fn select_proxy(&self, group_name: String, proxy_name: String) -> Result<(), String> {
        let body = serde_json::json!({
            "name": proxy_name
        });

        let path = format!("/proxies/{}", urlencoding::encode(&group_name));
        self.request_no_response("PUT", &path, Some(serde_json::to_vec(&body).unwrap()))
            .await
            .map_err(|e| format!("Failed to select proxy: {}", e))
    }

    /// Get active connections
    pub async fn get_connections(&self) -> Result<ConnectionsResponse, String> {
        self.request("GET", "/connections", None).await
            .map_err(|e| format!("Failed to get connections: {}", e))
    }

    /// Get current configuration
    pub async fn get_configs(&self) -> Result<ConfigResponse, String> {
        self.request("GET", "/configs", None).await
            .map_err(|e| format!("Failed to get config: {}", e))
    }

    /// Set proxy mode (rule, global, direct)
    pub async fn set_mode(&self, mode: Mode) -> Result<(), String> {
        let mode_str = match mode {
            Mode::Rule => "rule",
            Mode::Global => "global",
            Mode::Direct => "direct",
        };
        let mut config = HashMap::new();
        config.insert("mode".to_string(), mode_str.to_string());
        
        let body = serde_json::to_vec(&config).unwrap();
        self.request_no_response("PATCH", "/configs", Some(body))
            .await
            .map_err(|e| format!("Failed to set mode: {}", e))
    }

    /// Get current proxy mode
    pub async fn get_mode(&self) -> Result<Option<Mode>, String> {
        let config = self.get_configs().await?;
        Ok(config.mode)
    }

    async fn request_no_response(
        &self,
        method: &str,
        path: &str,
        body: Option<Vec<u8>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(unix)]
        if let Some(socket_path) = &self.socket_path {
            let client = Client::builder(TokioExecutor::new()).build(UnixConnector);
            let uri: hyper::Uri = UnixUri::new(socket_path, path).into();

            let request_builder = Request::builder()
                .uri(uri)
                .method(method)
                .header("Content-Type", "application/json");

            let request = if let Some(body_data) = body {
                request_builder.body(Full::new(Bytes::from(body_data)))?
            } else {
                request_builder.body(Full::new(Bytes::new()))?
            };

            let response = client.request(request).await?;

            if !response.status().is_success() {
                return Err(format!("HTTP status error: {}", response.status()).into());
            }

            return Ok(());
        }

        // HTTP fallback
        if let Some(http_url) = &self.http_url {
            let client = Client::builder(TokioExecutor::new()).build_http();
            let uri = format!("{}{}", http_url, path);

            let request_builder = Request::builder()
                .uri(uri)
                .method(method)
                .header("Content-Type", "application/json");

            let request = if let Some(body_data) = body {
                request_builder.body(Full::new(Bytes::from(body_data)))?
            } else {
                request_builder.body(Full::new(Bytes::new()))?
            };

            let response = client.request(request).await?;

            if !response.status().is_success() {
                return Err(format!("HTTP status error: {}", response.status()).into());
            }

            return Ok(());
        }

        Err("No connection method configured".into())
    }

    async fn request<T>(
        &self,
        method: &str,
        path: &str,
        body: Option<Vec<u8>>,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        #[cfg(unix)]
        if let Some(socket_path) = &self.socket_path {
            let uri: hyper::Uri = UnixUri::new(socket_path, path).into();
            let client = Client::builder(TokioExecutor::new()).build(UnixConnector);

            let request_builder = Request::builder()
                .uri(uri)
                .method(method)
                .header("Content-Type", "application/json");

            let request = if let Some(body_data) = body {
                request_builder.body(Full::new(Bytes::from(body_data)))?
            } else {
                request_builder.body(Full::new(Bytes::new()))?
            };

            let response = client.request(request).await?;

            if !response.status().is_success() {
                return Err(format!("HTTP status error: {}", response.status()).into());
            }

            let body_bytes = response
                .into_body()
                .collect()
                .await?
                .to_bytes();

            return Ok(serde_json::from_slice(&body_bytes)?);
        }

        // HTTP fallback
        if let Some(http_url) = &self.http_url {
            let client = Client::builder(TokioExecutor::new()).build_http();
            let uri = format!("{}{}", http_url, path);

            let request_builder = Request::builder()
                .uri(uri)
                .method(method)
                .header("Content-Type", "application/json");

            let request = if let Some(body_data) = body {
                request_builder.body(Full::new(Bytes::from(body_data)))?
            } else {
                request_builder.body(Full::new(Bytes::new()))?
            };

            let response = client.request(request).await?;

            if !response.status().is_success() {
                return Err(format!("HTTP status error: {}", response.status()).into());
            }

            let body_bytes = response
                .into_body()
                .collect()
                .await?
                .to_bytes();

            return Ok(serde_json::from_slice(&body_bytes)?);
        }

        Err("No connection method configured".into())
    }
}
