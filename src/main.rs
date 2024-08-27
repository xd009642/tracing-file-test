use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_sdk::trace::Config;
use opentelemetry::trace::TracerProvider as TracerProviderTrait;
use opentelemetry::{global, Key, KeyValue};
use opentelemetry_sdk::{
    metrics::{
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
        Aggregation, Instrument, MeterProviderBuilder, PeriodicReader, SdkMeterProvider, Stream,
    },
    runtime,
    trace::{BatchConfig, RandomIdGenerator, Sampler, Tracer},
    Resource,
};

fn init_tracer() -> Tracer {
    let exporter = opentelemetry_stdout::SpanExporter::default();

    let provider = TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    opentelemetry::global::set_tracer_provider(provider.clone());

    provider.tracer("tracing-otel-subscriber")
}
    
fn main() {
    let tracer = init_tracer();

    let filter = tracing_subscriber::filter::EnvFilter::from_default_env()
        .add_directive(tracing_subscriber::filter::LevelFilter::TRACE.into())
        .add_directive(
            "polling::epoll=OFF"
                .parse()
                .expect("Failed to setup filter for polling::epoll"),
        );

    let otel_layer = OpenTelemetryLayer::new(tracer);

    let fmt = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(std::fs::File::create("traces").expect("Failed to create file"));

    tracing_subscriber::registry()
        .with(filter)
        .with(otel_layer)
        .with(fmt)
        .init();


    foo();

    opentelemetry::global::shutdown_tracer_provider();
}


#[tracing::instrument]
fn foo() {
    tracing::info!(
        monotonic_counter.foo = 1_u64,
        key_1 = "bar",
        key_2 = 10,
        "handle foo",
    );

    tracing::info!(histogram.baz = 10, "histogram example",);
}
