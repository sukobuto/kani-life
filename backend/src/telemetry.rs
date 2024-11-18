use opentelemetry::trace::{TraceError, TracerProvider as _};
use opentelemetry::KeyValue;
use opentelemetry_sdk::trace::RandomIdGenerator;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_sdk::{trace as sdktrace, trace::TracerProvider};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing::Subscriber;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

fn init_tracer_provider() -> Result<TracerProvider, TraceError> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()?;

    Ok(TracerProvider::builder()
        .with_batch_exporter(exporter, runtime::Tokio)
        .with_config(
            sdktrace::Config::default()
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![KeyValue::new(
                    SERVICE_NAME,
                    "kani-life",
                )])),
        )
        .build())
}

fn init_tracer(name: &str) -> sdktrace::Tracer {
    let provider = init_tracer_provider().expect("Failed to initialize tracer provider.");
    provider.tracer(name.to_owned())
}

pub fn init_tracing_subscriber(name: &str) -> impl Subscriber + Send + Sync + 'static {
    let tracer = init_tracer(name);
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    subscriber
}
