use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, propagation::TraceContextPropagator, trace::SdkTracerProvider};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::{OtlpProtocol, TelemetryConfig},
    core::error::{ApiError, Result},
};

pub struct ApiTelemetry {
    provider: Option<SdkTracerProvider>,
}

impl ApiTelemetry {
    pub fn init(config: TelemetryConfig) -> Result<Self> {
        let service_name = config.resolved_service_name();
        let otlp_endpoint = config.resolved_otlp_endpoint();
        let otlp_protocol = config.resolved_otlp_protocol();

        global::set_text_map_propagator(TraceContextPropagator::new());

        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let fmt_layer = fmt::layer().with_target(true);
        let registry = tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer);

        let mut provider_handle = None;

        if let Some(endpoint) = otlp_endpoint {
            let exporter = match otlp_protocol {
                OtlpProtocol::Grpc => SpanExporter::builder()
                    .with_tonic()
                    .with_endpoint(endpoint)
                    .build()
                    .map_err(|e| ApiError::Telemetry(e.to_string()))?,
                OtlpProtocol::Http => SpanExporter::builder()
                    .with_http()
                    .with_endpoint(endpoint)
                    .build()
                    .map_err(|e| ApiError::Telemetry(e.to_string()))?,
            };

            let provider = SdkTracerProvider::builder()
                .with_batch_exporter(exporter)
                .with_resource(
                    Resource::builder_empty()
                        .with_service_name(service_name.clone())
                        .build(),
                )
                .build();

            let tracer = provider.tracer(service_name);
            global::set_tracer_provider(provider.clone());

            let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            registry
                .with(otel_layer)
                .try_init()
                .map_err(|e| ApiError::Telemetry(e.to_string()))?;

            provider_handle = Some(provider);
        } else {
            registry
                .try_init()
                .map_err(|e| ApiError::Telemetry(e.to_string()))?;
        }

        Ok(Self {
            provider: provider_handle,
        })
    }

    pub fn shutdown(&self) {
        if let Some(provider) = &self.provider {
            tracing::info!("Flushing remaining OpenTelemetry spans...");

            if let Err(e) = provider.shutdown() {
                eprintln!("Error during telemetry shutdown: {e}");
            }
        }
    }
}

impl Drop for ApiTelemetry {
    fn drop(&mut self) {
        self.shutdown();
    }
}
