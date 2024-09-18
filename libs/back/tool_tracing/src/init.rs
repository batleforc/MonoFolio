#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
use super::tracing_kind::{Tracing, TracingKind};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Config;
use opentelemetry_sdk::{runtime, Resource};
use std::env;
use std::{fs::File, sync::Arc, vec};
use time::format_description;
use tracing::level_filters::LevelFilter;
use tracing::subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Layer, Registry};

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn init_tracing(tracing_config: Vec<Tracing>, name: String) {
    let mut layers = vec![];
    for config in tracing_config {
        match config.kind {
            TracingKind::File => {
                let file = File::options()
                    .create(true)
                    .truncate(false)
                    .append(true)
                    .open("trace.log")
                    .expect("Failed to create trace.log");
                let formating_layer =
                    BunyanFormattingLayer::new(name.clone(), Arc::new(file)).boxed();
                layers.push(JsonStorageLayer.boxed());
                layers.push(formating_layer);
            }
            TracingKind::Console => {
                let time_format = format_description::parse("[hour]:[minute]:[second]")
                    .expect("format string should be valid!");
                let timer = UtcTime::new(time_format);
                let env_filter = EnvFilter::builder()
                    .with_default_directive(LevelFilter::from(config.level).into())
                    .from_env()
                    .unwrap()
                    .add_directive("serenity=error".parse().unwrap());
                let terminal_out = fmt::layer()
                    .with_thread_names(true)
                    .with_timer(timer)
                    .with_target(false)
                    .with_filter(env_filter)
                    .boxed();
                layers.push(terminal_out);
            }
            TracingKind::Otel => {
                let endpoint = match config.additional.get("endpoint") {
                    Some(endpoint) => endpoint.to_string(),
                    None => "http://localhost:4317".to_string(),
                };
                let endpoint_from_env = env::var(format!(
                    "{}_OTEL_EXPORTER_OTLP_ENDPOINT",
                    config.name.to_uppercase()
                ))
                .unwrap_or(endpoint);
                let pod_name =
                    std::env::var("POD_NAME").unwrap_or_else(|_| "not_a_pod".to_string());
                println!(
                    "Connecting to endpoint: {} with ENV {}_OTEL_EXPORTER_OTLP_ENDPOINT",
                    endpoint_from_env.clone(),
                    config.name.to_uppercase()
                );
                let telemetry = opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(
                        opentelemetry_otlp::new_exporter()
                            .tonic()
                            .with_endpoint(endpoint_from_env),
                    )
                    .with_trace_config(Config::default().with_resource(Resource::new(vec![
                        KeyValue::new("service.name", name.clone()),
                        KeyValue::new("service.pod", pod_name.clone()),
                    ])))
                    .install_batch(runtime::Tokio)
                    .expect("Failed to install opentelemetry");
                let env_filter = EnvFilter::builder()
                    .with_default_directive(LevelFilter::from(config.level).into())
                    .from_env()
                    .unwrap();
                let tele_layer = OpenTelemetryLayer::new(telemetry).with_filter(env_filter);
                layers.push(tele_layer.boxed());
            }
        }
    }
    subscriber::set_global_default(Registry::default().with(layers))
        .expect("setting default subscriber failed");
}

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn stop_tracing(tracing_config: Vec<Tracing>, _name: String) {
    if tracing_config.iter().any(|x| x.kind == TracingKind::Otel) {
        opentelemetry::global::shutdown_tracer_provider();
    }
}

#[cfg(test)]
mod tests {
    use crate::level::VerboseLevel;

    use super::*;
    use std::fs::remove_file;

    #[tokio::test]
    async fn test_init_tracing() {
        let mut tracing_config = vec![
            Tracing {
                kind: TracingKind::File,
                level: VerboseLevel::DEBUG,
                additional: Default::default(),
                name: "test1".to_string(),
            },
            Tracing {
                kind: TracingKind::Console,
                level: VerboseLevel::INFO,
                additional: Default::default(),
                name: "test2".to_string(),
            },
            Tracing {
                kind: TracingKind::Otel,
                level: VerboseLevel::DEBUG,
                additional: Default::default(),
                name: "test3".to_string(),
            },
        ];
        tracing_config[2]
            .additional
            .insert("endpoint".to_string(), "http://localhost:4317".to_string());
        init_tracing(tracing_config.clone(), "test".to_string());
        tracing::info!("test part of test_init_tracing");
        tracing::error!("test part of test_init_tracing");
        tokio::spawn(async {
            stop_tracing(tracing_config, "test".to_string());
        });
        remove_file("trace.log").unwrap();
    }
}
