use prometheus::{Encoder, TextEncoder, CounterVec, register_counter_vec};
use std::sync::Arc;
use warp::Filter;

// Define a struct to hold our error metrics
#[derive(Clone)]
struct ErrorMetrics {
    error_counter: Arc<CounterVec>,
}

impl ErrorMetrics {
    // Initialize the error metrics
    fn new() -> Self {
        let error_counter = register_counter_vec!(
            "app_errors_total",
            "Total number of errors categorized by type and source",
            &["error_type", "source"]
        ).expect("Failed to create error counter");

        Self {
            error_counter: Arc::new(error_counter),
        }
    }

    // Function to record an error
    fn record_error(&self, error_type: &str, source: &str) {
        self.error_counter
            .with_label_values(&[error_type, source])
            .inc();
    }

    // Function to serve the Prometheus metrics over HTTP
    async fn serve_metrics(&self) {
        // Define the Warp route for /metrics
        let metrics_route = warp::path("metrics").map(move || {
            let mut buffer = Vec::new();
            let encoder = TextEncoder::new();  // Create the encoder inside the closure
            let metric_families = prometheus::gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            warp::reply::with_header(buffer, "Content-Type", encoder.format_type())
        });

        // Start the server on port 3030
        warp::serve(metrics_route)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }
}

#[tokio::main]
async fn main() {
    let metrics = ErrorMetrics::new();

    // Simulate recording some errors
    metrics.record_error("database_error", "db_service");
    metrics.record_error("http_error", "web_service");

    // Serve the metrics
    println!("Serving metrics on http://127.0.0.1:3030/metrics");
    metrics.serve_metrics().await;
}
