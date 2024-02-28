use act_zero::runtimes::tokio::spawn_actor;
use act_zero::*;
use tokio::time::Instant;
use tracing::{info, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::util::SubscriberInitExt as _;

#[derive(Debug, Clone)]
struct ShortGreeter {
    inner: Addr<InnerGreeterActor>,
    start: Instant,
    number_of_greets: i32,
}

#[derive(Debug, Clone)]
struct InnerGreeterActor {}

#[derive(Debug, Clone)]
struct InnerGreeterNormal {}

impl Actor for ShortGreeter {}
impl Actor for InnerGreeterActor {}

#[derive(Debug, Clone)]
pub struct Context {
    parent: Span,
}

impl Context {
    pub fn update(mut self) -> Self {
        self.parent = Span::current();
        self
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            parent: Span::current(),
        }
    }
}

impl InnerGreeterActor {
    #[tracing::instrument(parent = &ctx.parent, skip_all)]
    async fn inner_actor(&mut self, ctx: Context, me: Addr<Self>, msg: String) -> ActorResult<()> {
        let ctx = ctx.update();
        tracing::info!("[ACTOR] Inner Greeter {msg}");

        let inner = InnerGreeterNormal::new();
        inner
            .inner_normal(ctx.clone(), "FROM_INNER_ACTOR".to_string())
            .await;

        send!(me.inner_actor_last(ctx, "FROM INNER ACTOR".to_string()));

        Produces::ok(())
    }

    // #[tracing::instrument(parent = &ctx.parent, skip_all)]
    #[tracing::instrument(skip_all)]
    async fn inner_actor_last(&mut self, ctx: Context, msg: String) {
        // test `set_parent also works`
        Span::current().set_parent(ctx.parent.context());
        tracing::info!("[ACTOR LAST] Grandfather {msg}");
    }

    #[tracing::instrument]
    fn new() -> Self {
        Self {}
    }
}

impl InnerGreeterNormal {
    #[tracing::instrument]
    fn new() -> Self {
        Self {}
    }

    #[tracing::instrument(parent = &ctx.parent, skip_all)]
    async fn inner_normal(self, ctx: Context, msg: String) {
        tracing::info!("[NORMAL] Inner Greeter {msg}");
    }
}

impl ShortGreeter {
    #[tracing::instrument(skip_all)]
    async fn short_greet(&mut self, msg: String) -> ActorResult<()> {
        self.number_of_greets += 1;

        let ctx = Context::default();
        let inner_adr = self.inner.clone();
        call!(self
            .inner
            .inner_actor(ctx.clone(), inner_adr, "FROM_SHORT_GREETER".to_string()));

        let inner = InnerGreeterNormal::new();
        inner
            .inner_normal(Context::default(), "FROM_SHORT_GREETER".to_string())
            .await;

        info!(
            "Short Greeter: Number {}, Since Start: {}ms, msg: {msg}",
            self.number_of_greets,
            self.start.elapsed().as_millis()
        );

        Produces::ok(())
    }

    #[tracing::instrument]
    fn new(inner: Addr<InnerGreeterActor>) -> Self {
        Self {
            inner,
            start: Instant::now(),
            number_of_greets: 0,
        }
    }
}

pub fn install_tracing() {
    use opentelemetry::trace::TracerProvider as _;
    use opentelemetry_sdk::trace::TracerProvider;
    use tracing_error::ErrorLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::EnvFilter;

    let exporter = opentelemetry_stdout::SpanExporter::default();
    let provider = TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();

    let tracer = provider.tracer("readme_example");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let fmt_layer = tracing_subscriber::fmt::layer();

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap()
        .add_directive("gst_utils=debug".parse().unwrap())
        .add_directive("timeline=info".parse().unwrap())
        .add_directive("state=info".parse().unwrap())
        .add_directive("opset=warn".parse().unwrap());

    tracing_subscriber::registry()
        .with(telemetry)
        .with(filter_layer)
        .with(ErrorLayer::default())
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() {
    install_tracing();

    let inner_greeter = spawn_actor(InnerGreeterActor::new());
    let short_greeter = spawn_actor(ShortGreeter::new(inner_greeter.clone()));

    {
        let short_greeter = short_greeter.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                call!(short_greeter.short_greet("FROM_TOKIO_TASK".to_string()));
            }
        })
    };

    let short_greeter = short_greeter.clone();
    send!(short_greeter.short_greet("FROM_MAIN".to_string()));

    let inner_normal = InnerGreeterNormal::new();
    inner_normal
        .inner_normal(Context::default(), "FROM_MAIN".to_string())
        .await;

    // short_greeter.termination().await
}
