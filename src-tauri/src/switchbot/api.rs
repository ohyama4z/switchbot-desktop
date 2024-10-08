use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use reqwest;
use reqwest::header::HeaderMap;
use reqwest::header::AUTHORIZATION;
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

pub(crate) const SWITCHBOT_API_URL: &str = "https://api.switch-bot.com/v1.1";

pub(crate) fn create_header(token: &str, secret: &str) -> HeaderMap {
    let t = Utc::now().timestamp_millis();
    let nonce = Uuid::new_v4().to_string();
    let sign = {
        let key = hmac::Key::new(ring::hmac::HMAC_SHA256, secret.as_bytes());
        let data = format!("{}{}{}", token, t, nonce);
        let sign = hmac::sign(&key, data.as_bytes());
        general_purpose::STANDARD.encode(sign.as_ref())
    };

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, token.parse().unwrap());
    headers.insert("sign", sign.parse().unwrap());
    headers.insert("t", t.to_string().parse().unwrap());
    headers.insert("nonce", nonce.parse().unwrap());

    headers
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub hub_device_id: String,
    pub enable_cloud_service: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfraredRemote {
    pub device_id: String,
    pub device_name: String,
    pub hub_device_id: String,
    pub remote_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceResponseBody {
    pub device_list: Vec<Device>,
    pub infrared_remote_list: Vec<InfraredRemote>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetDeviceResponse {
    status_code: u16,
    body: GetDeviceResponseBody,
    message: String,
}

pub async fn get_devices(
    token: &str,
    secret: &str,
) -> Result<GetDeviceResponseBody, Box<dyn Error>> {
    let url = format!("{}/devices", SWITCHBOT_API_URL);
    let headers = create_header(token, secret);
    let client = reqwest::Client::new();

    let res = client.get(&url).headers(headers).send().await?;
    let get_device_response: GetDeviceResponse = res.json().await?;

    Ok(get_device_response.body)
}

#[derive(Debug, Clone, Serialize, Deserialize, strum::EnumString)]
pub(crate) enum Parameter {
    #[strum(serialize = "default")]
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommandBody {
    pub(crate) command: String,
    pub(crate) command_type: String,
    pub(crate) parameter: Parameter,
}

pub(crate) async fn send_command(
    token: &str,
    secret: &str,
    device_id: &str,
    body: CommandBody,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/devices/{}/commands", SWITCHBOT_API_URL, device_id);
    let headers = create_header(token, secret);
    let client = reqwest::Client::new();

    let res = client
        .post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;
    let _ = res.text().await?;

    Ok(())
}
