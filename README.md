## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# err_metrics

`err_metrics` is a Rust library and command-line tool for tracking and exposing error metrics in Prometheus format. It uses asynchronous support from Tokio and serves metrics over HTTP using Warp, making it easy to integrate with observability platforms like Prometheus and Grafana.

---

## Features
- **Error Counting**: Categorize and count errors based on type and source.
- **Prometheus Integration**: Exposes error metrics in a format that Prometheus can scrape.
- **Async Support**: Designed to work efficiently in asynchronous Rust applications using Tokio.
- **Lightweight HTTP Server**: Uses Warp to serve metrics on a configurable endpoint.

---

## Installation
Add `err_metrics` to your `Cargo.toml`:

```toml
[dependencies]
err_metrics = "0.1.0"  # Replace with the correct version
```
Note: Make sure to use the latest version published on crates.io.

## Quick Start
Here's a simple example to get you started:

```rust
use err_metrics::ErrorMetrics;
use tokio::main;

#[main]
async fn main() {
    let metrics = ErrorMetrics::new();

    // Simulate some errors
    metrics.record_error("database_error", "db_service");
    metrics.record_error("http_error", "web_service");

    // Serve metrics on http://127.0.0.1:3030/metrics
    println!("Serving metrics on http://127.0.0.1:3030/metrics");
    metrics.serve_metrics().await;
}
```
## Run the example:
bash

cargo run
Visit http://127.0.0.1:3030/metrics to see the metrics.

## Usage
Recording Errors
You can record errors using the record_error method, specifying the error type and source:
metrics.record_error("database_error", "db_service");
metrics.record_error("http_error", "web_service");

## Exposing Metrics
err_metrics exposes a /metrics endpoint that Prometheus can scrape. The endpoint is served using Warp on a configurable port (default: 3030).

## Integration with Prometheus
To monitor your error metrics using Prometheus:

## Add a scrape job to your Prometheus configuration:
```yaml
scrape_configs:
  - job_name: 'err_metrics'
    static_configs:
      - targets: ['127.0.0.1:3030']
```
## Configuration
You can easily customize the HTTP server port and other settings by modifying the serve_metrics method or using environment variables (future feature).

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements
Prometheus: For providing a powerful metrics collection system.
Tokio: For async runtime support.
Warp: For the fast and easy-to-use HTTP server.

##Author
Ben Santora 
