use std::error::Error;
use std::time::Duration;
use log::info;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::sleep;

mod web_requests;
mod web_server;
mod prometheus;
pub mod config_file;

use prometheus::metrics::METRICS;

pub async fn get_web_metrics(
    endpoints: Vec<String>,
    port: u16,
    retry_seconds: u64
) -> Result<(), Box<dyn Error>> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let server_handle:JoinHandle<Result<(), hyper::Error>> = tokio::spawn(web_server::metrics_server(
        port,
        shutdown_rx
    ));

    tokio::spawn(async move {
        loop {
            for endpoint in &endpoints {
                let (metric_status_code, metrics_duration) :(u16, Duration) = web_requests::get_status_and_time(endpoint)
                    .await
                    .expect("Unable to get metrics of url");

                METRICS.response_time.with_label_values(&[endpoint]).set(metrics_duration.as_secs_f64());
                METRICS.status_code.with_label_values(&[endpoint, &metric_status_code.to_string()]).inc();

                info!("Update metrics for {}: status_code = {:?}, response_time = {:?}", endpoint, metric_status_code, metrics_duration);
            }
            sleep(Duration::from_secs(retry_seconds)).await;
        }
    });

    tokio::signal::ctrl_c().await.expect("failed to listen for event");
    let _ = shutdown_tx.send(());
    server_handle.await??;

    Ok(())
}
