use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{
    Body,
    Request,
    Response,
    Server
};
use hyper::server::conn::AddrStream;
use hyper::service::{
    make_service_fn,
    service_fn
};

use prometheus::{
    Encoder,
    TextEncoder
};

use tokio::sync::oneshot;
use log::info;
use prometheus::proto::MetricFamily;

pub async fn metrics_server(
    port :u16,
    shutdown_rx: oneshot::Receiver<()>
) -> Result<(), hyper::Error> {
    let addr :SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_svc = make_service_fn(|_conn: &AddrStream| {
        async {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                async move {
                    match req.uri().path() {
                        "/metrics" => {
                            let encoder :TextEncoder = TextEncoder::new();
                            let metric_families :Vec<MetricFamily> = prometheus::gather();
                            let mut buffer :Vec<u8> = Vec::new();
                            encoder.encode(&metric_families, &mut buffer).unwrap();

                            info!("Metrics endpoint accessed");

                            Ok::<_, Infallible>(Response::builder()
                                .header(hyper::header::CONTENT_TYPE, encoder.format_type())
                                .body(Body::from(buffer))
                                .unwrap())

                        }
                        "/health" => {
                            info!("Health endpoint accessed");
                            Ok::<_, Infallible>(Response::new(Body::from("OK")))
                        }
                        _ => {
                            Ok::<_, Infallible>(Response::builder()
                                .status(404)
                                .body(Body::from("Not Found"))
                                .unwrap())
                        }
                    }
                }
            }))
        }
    });

    Server::bind(&addr).serve(make_svc).with_graceful_shutdown(async {
        shutdown_rx.await.ok();
    }).await
}
