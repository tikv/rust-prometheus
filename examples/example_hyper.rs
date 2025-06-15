// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::net::SocketAddr;
use std::sync::LazyLock;

use hyper::body::Incoming;
use hyper::header::CONTENT_TYPE;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Request;
use hyper::Response;
use hyper_util::rt::TokioIo;
use prometheus::{labels, opts, register_counter, register_gauge, register_histogram_vec};
use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};
use tokio::net::TcpListener;

type BoxedErr = Box<dyn std::error::Error + Send + Sync + 'static>;

static HTTP_COUNTER: LazyLock<Counter> = LazyLock::new(|| {
    register_counter!(opts!(
        "example_http_requests_total",
        "Number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap()
});
static HTTP_BODY_GAUGE: LazyLock<Gauge> = LazyLock::new(|| {
    register_gauge!(opts!(
        "example_http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap()
});
static HTTP_REQ_HISTOGRAM: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap()
});

async fn serve_req(_req: Request<Incoming>) -> Result<Response<String>, BoxedErr> {
    let encoder = TextEncoder::new();

    HTTP_COUNTER.inc();
    let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();

    let metric_families = prometheus::gather();
    let body = encoder.encode_to_string(&metric_families)?;
    HTTP_BODY_GAUGE.set(body.len() as f64);

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(body)?;

    timer.observe_duration();

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), BoxedErr> {
    let addr: SocketAddr = ([127, 0, 0, 1], 9898).into();
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let service = service_fn(serve_req);
        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            eprintln!("server error: {:?}", err);
        };
    }
}
