#[macro_use]
extern crate tracing;

use opentelemetry_contrib::trace::exporter::datadog;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    let _uninstall = create_datadog_tracing_subscriber("http://localhost:8126");
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");
    tokio::time::sleep(std::time::Duration::from_millis(1_000)).await;
}

fn create_datadog_tracing_subscriber(agent: &str) -> datadog::Uninstall {
    let (tracer, uninstall) = datadog::new_pipeline()
        .with_service_name("datadogtest")
        .with_version(datadog::ApiVersion::Version05)
        .with_agent_endpoint(agent)
        .install()
        .unwrap();

    // integration with `tracing`
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = tracing_subscriber::Registry::default().with(telemetry);

    // Make the datadog subscriber the default one throughout the program
    tracing::subscriber::set_global_default(subscriber).unwrap();

    uninstall
}
