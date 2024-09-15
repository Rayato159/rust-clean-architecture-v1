use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{SpanKind, Status};
use opentelemetry::{global, trace::Tracer};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout::SpanExporter;

pub fn init_tracer() {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = TracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(provider);
}

pub fn tracing_span(fn_name: String, e: Option<String>) {
    let tracer = global::tracer("main");

    let mut span = tracer
        .span_builder(fn_name)
        .with_kind(SpanKind::Server)
        .start(&tracer);

    if let Some(e) = e {
        span.set_status(Status::error(e.to_string()));
    } else {
        span.set_status(Status::Ok);
    }
}
