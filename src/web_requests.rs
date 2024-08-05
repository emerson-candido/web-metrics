use reqwest::{
    Client,
    Response
};

use std::time::{
    Duration,
    Instant
};

use std::error::Error;

pub async fn get_status_and_time(
    url: &str
) -> Result<(u16, Duration), Box<dyn Error>> {
    let client :Client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let start :Instant = Instant::now();

    let response :Response = client.get(url).send().await?;

    let duration :Duration = start.elapsed();
    let status :u16 = response.status().as_u16();

    Ok((status, duration))
}
