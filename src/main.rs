use serde::{Serialize};
use reqwest::{Client};
use std::{env, error, thread, time};
use dotenv::dotenv;

struct Config {
    pub zone_id: String,
    pub dns_record: String,
    pub cf_token: String,
    pub poll_interval: u64,
}

#[derive(Serialize)]
struct DnsUpdate {
    r#type: String,
    name: String,
    content: String,
    ttl: u32,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn error::Error>> {
        let zone_id = env::var("CLOUDFLARE_ZONE")?;
        let dns_record = env::var("CLOUDFLARE_DNS_RECORD")?;
        let cf_token = env::var("CLOUDFLARE_TOKEN")?;
        let poll_interval = env::var("POLL_INTERVAL")?.parse()?;
        Ok(Config {
            zone_id,
            dns_record,
            cf_token,
            poll_interval,
        })
    }
}


async fn get_ip(client: &Client) -> Result<String, Box<dyn error::Error>> {
    let resp: serde_json::Value = client.get("https://api.ipify.org?format=json")
        .send()
        .await?
        .json()
        .await?;
    Ok(resp["ip"].to_string())
}

async fn get_cf_ip(client: &Client, config: &Config) -> Result<String, Box<dyn error::Error>> {
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", config.zone_id, config.dns_record);
    let resp: serde_json::Value = client.get(&url)
        .header("Authorization", format!("Bearer {}", config.cf_token))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json()
        .await?;
    println!("Current Cloudflare IP address: {}", resp["result"]["content"]);
    Ok(resp["result"]["content"].to_string())
}

async fn set_ip(client: &Client, config: &Config, ip: &String) -> Result<serde_json::Value, Box<dyn error::Error>> {
    let body = DnsUpdate {
        r#type: String::from("A"),
        name: String::from("home.wangfu.org"),
        content: ip.to_string(),
        ttl: 120,
    };
    let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", config.zone_id, config.dns_record);
    let resp: serde_json::Value = client.put(&url)
        .header("Authorization", format!("Bearer {}", config.cf_token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?
        .json()
        .await?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();
    let config = Config::new()?;
    let client = Client::new();
    let mut current_ip = get_cf_ip(&client, &config).await?;
    loop {
        let ip = get_ip(&client).await?;
        if ip != current_ip {
            println!("New IP address detected: {}", ip);
            let result = set_ip(&client, &config, &ip).await?;
            println!("Cloudflare updated with new IP: {}. Sleeping for {} seconds", result["result"]["content"], config.poll_interval);
            current_ip = ip;
        } else {
            println!("IP address has not changed. Sleeping for {} seconds", config.poll_interval);
        }
        thread::sleep(time::Duration::from_secs(config.poll_interval));
    }
}