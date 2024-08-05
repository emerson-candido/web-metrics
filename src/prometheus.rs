use prometheus::{
    GaugeVec,
    IntCounterVec,
    register_gauge_vec,
    register_int_counter_vec
};

use once_cell::sync::Lazy;

pub struct Metrics {
    pub response_time: GaugeVec,
    pub status_code: IntCounterVec,
}

impl Metrics {
    pub fn new() -> Self {
        let response_time = register_gauge_vec!(
            "http_request_duration_seconds",
            "HTTP request duration in seconds",
            &["endpoint"]
        ).unwrap();

        let status_code = register_int_counter_vec!(
            "http_response_status_codes_total",
            "HTTP response status codes",
            &["endpoint", "status"]
        ).unwrap();

        Metrics {
            response_time,
            status_code,
        }
    }
}

pub mod metrics {
    use super::*;

    pub static METRICS: Lazy<Metrics> = Lazy::new(Metrics::new);
}
