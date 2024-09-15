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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Device {
    device_id: String,
    device_name: String,
    device_type: String,
    hub_device_id: String,
    enable_cloud_service: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InfraredRemote {
    device_id: String,
    device_name: String,
    hub_device_id: String,
    remote_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetDeviceResponseBody {
    device_list: Vec<Device>,
    inframed_remote_list: Vec<InfraredRemote>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetDeviceResponse {
    status_code: u16,
    body: GetDeviceResponseBody,
    message: String,
}

pub(crate) async fn get_devices(token: &str, secret: &str) -> Result<GetDeviceResponseBody, Error> {
    let url = format!("{}/devices", SWITCHBOT_API_URL);
    let headers = create_header(token, secret);
    let client = reqwest::Client::new();

    let res = client.get(&url).headers(headers).send().await?;
    let get_device_response: GetDeviceResponse = res.json().await?;

    Ok(get_device_response.body)
}

#[derive(Debug, Clone)]
pub(crate) enum Parameter {
    Default = "default",
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommandBody {
    command: String,
    command_type: String,
    parameter: Parameter,
}

pub(crate) async fn send_command(
    token: &str,
    secret: &str,
    device_id: &str,
    body: CommandBody,
) -> Result<(), Error> {
    let url = format!("{}/devices/{}/command", SWITCHBOT_API_URL, device_id);
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
