use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn init_tracer(_args: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input TokenStream as a function
    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;

    let statements = block.stmts;

    // Generate the tracer initialization code inside the function
    let result = quote! {
        use opentelemetry::global;
        use opentelemetry_sdk::propagation::TraceContextPropagator;
        use opentelemetry_sdk::trace::TracerProvider;
        use opentelemetry_stdout::SpanExporter;

        // Function definition with tracer initialization
        #(#attrs)*  // Retain any attributes on the original function

        #vis #sig {
            // Initialize the tracer before running the function body
            global::set_text_map_propagator(TraceContextPropagator::new());
            let provider = TracerProvider::builder()
                .with_simple_exporter(SpanExporter::default())
                .build();
            global::set_tracer_provider(provider);

            // Original function body
            #(#statements)*
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn tracing_execution(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;

    let fn_name = sig.ident.to_string();
    let statements = block.stmts;

    let is_async = sig.asyncness.is_some();

    let result = if is_async {
        quote! {
            #(#attrs)*

            #vis #sig {
                let tracer = opentelemetry::global::tracer(#fn_name);
                let mut span = tracer
                    .span_builder(#fn_name)
                    .with_kind(opentelemetry::trace::SpanKind::Server)
                    .start(&tracer);

                let _start = std::time::Instant::now();

                let result = async {
                    #(#statements)*
                }.await;

                let duration = _start.elapsed().as_millis();
                span.set_attribute(opentelemetry::KeyValue::new("duration_ms", duration as i64));

                span.set_status(opentelemetry::trace::Status::Ok);
                span.end();

                result
            }
        }
    } else {
        quote! {
            #(#attrs)*

            #vis #sig {
                let tracer = opentelemetry::global::tracer(#fn_name);
                let mut span = tracer
                    .span_builder(#fn_name)
                    .with_kind(opentelemetry::trace::SpanKind::Server)
                    .start(&tracer);

                let _start = std::time::Instant::now();
                let result = (|| {
                    #(#statements)*
                })();

                let duration = _start.elapsed().as_millis();
                span.set_attribute(opentelemetry::KeyValue::new("duration_ms", duration as i64));

                span.set_status(opentelemetry::trace::Status::Ok);
                span.end();

                result
            }
        }
    };

    result.into()
}
