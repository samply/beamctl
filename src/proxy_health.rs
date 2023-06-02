use std::{time::SystemTime, process::exit};

use anyhow::{Result, Context};
use reqwest::{Client, StatusCode, Url};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStatus {
    last_active: SystemTime
}

pub async fn query_proxy_health(proxy_name: &str, api_key: &str, broker_url: &Url) -> Result<()> {
    let client = Client::new();
    let mut url = broker_url.clone();
    url.set_path(&format!("v1/health/{proxy_name}"));
    let req = client.get(url).basic_auth("", Some(api_key)).build().context("Failed to build request")?;
    let res = client.execute(req).await.context("Failed to execute request")?;
    match res.status() {
        StatusCode::SERVICE_UNAVAILABLE => {
            println!("Proxy {proxy_name} unavalible!");
            exit(2);
        },
        StatusCode::UNAUTHORIZED => {
            println!("Invalid monitoring apikey!");
            exit(13);
        },
        StatusCode::OK => {
            let status: ProxyStatus = res.json().await.unwrap();
            let last_report_dur = status.last_active.elapsed().unwrap();
            let minutes = last_report_dur.as_secs() / 60;
            let seconds = last_report_dur.as_secs() % 60;
            println!("Proxy reported back {minutes}m and {seconds}s ago!");
            exit(0);
        },
        unexpectd => {
            println!("Got unexpected statuscode {unexpectd} from broker");
            exit(13);
        }
    }
}
