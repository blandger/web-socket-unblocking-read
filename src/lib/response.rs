use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KucoinPublicTokenResponse {
    pub code: String,
    pub data: KucoinPublicTokenResponseData,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KucoinPublicTokenResponseData {
    pub token: String,
    pub instance_servers: Vec<KucoinPublicTokenInstanceServer>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KucoinPublicTokenInstanceServer {
    pub endpoint: String,
    pub encrypt: bool,
    pub protocol: String,
    pub ping_interval: u16,
    pub ping_timeout: u16,
}