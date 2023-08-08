use beam_lib::{TaskRequest, FailureStrategy, MsgId, AppId, TaskResult};
use clap::Args;
use monitoring_lib::Check;
use anyhow::{Result, bail};
use reqwest::{Url, header, StatusCode};
use serde_json::Value;

use crate::icinga::IcingaCode;

#[derive(Debug, Args)]
pub struct BridgeheadCheck {
    #[arg(long, env, value_parser)]
    beam_proxy_url: Url,

    /// Beam app id of this application
    #[arg(long, env, value_parser)]
    beam_proxy_name: String,

    // Beam app secret of this application
    #[arg(long, env, value_parser)]
    beam_proxy_secret: String,

    #[arg(value_parser = parse_check)]
    checks: Vec<Check>,

    /// Receiving bridgehead-monitoring beam id
    #[arg(long, value_parser)]
    to: String
}

pub fn parse_check(input: &str) -> Result<Check> {
    Ok(serde_json::from_value(Value::String(input.to_owned()))?)
}

pub async fn check_bridgehead(
    BridgeheadCheck {
        beam_proxy_url,
        beam_proxy_name,
        beam_proxy_secret,
        checks,
        to
    }: BridgeheadCheck
) -> Result<IcingaCode> {
    let client = reqwest::Client::new();
    let task = TaskRequest {
        id: MsgId::new(),
        from: AppId::new_unchecked(&beam_proxy_name),
        to: vec![AppId::new_unchecked(to)],
        body: checks.clone(),
        ttl: "60s".to_string(),
        failure_strategy: FailureStrategy::Discard,
        metadata: Value::Null,
    };
    let res = client
        .post(beam_proxy_url.join("/v1/tasks")?)
        .header(header::AUTHORIZATION, format!("ApiKey {} {}", beam_proxy_name, beam_proxy_secret))
        .json(&task)
        .send()
        .await?;
    if res.status() != StatusCode::CREATED {
        bail!("Failed to create task: Got status {}", res.status());
    }
    let res = client
        .get(beam_proxy_url.join(&format!("/v1/tasks/{}/results?wait_count=1", task.id))?)
        .header(header::AUTHORIZATION, format!("ApiKey {} {}", beam_proxy_name, beam_proxy_secret))
        .send()
        .await?;
    if res.status() != StatusCode::OK {
        bail!("Failed to retrive task: Got status {}", res.status());
    }
    let results = res.json::<Vec<TaskResult<Vec<String>>>>().await?.pop();
    match results {
        Some(task) => checks.into_iter().zip(task.body).for_each(|(check, result)| println!("{check}: {result}")),
        None => bail!("Got no results from task"),
    };
    Ok(IcingaCode::Ok)
}
