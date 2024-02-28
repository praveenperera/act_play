use act_zero::runtimes::tokio::spawn_actor;
use act_zero::*;
use tokio::time::Instant;
use tracing::{info, Span};

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
    parent_span: Span,
}

impl Context {
    pub fn update(mut self) -> Self {
        self.parent_span = Span::current();
        self
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            parent_span: Span::current(),
        }
    }
}

impl InnerGreeterActor {
    #[tracing::instrument(parent = &ctx.parent_span, skip_all)]
    async fn inner_actor(&mut self, ctx: Context, me: Addr<Self>) -> ActorResult<()> {
        let ctx = ctx.update();
        tracing::info!("[ACTOR] Inner Greeter");

        let inner = InnerGreeterNormal::new();
        inner.inner_normal(ctx.clone()).await;

        send!(me.inner_actor_last(ctx));

        Produces::ok(())
    }

    #[tracing::instrument(parent = &ctx.parent_span, skip_all)]
    async fn inner_actor_last(&mut self, ctx: Context) {
        tracing::info!("[ACTOR LAST] Grandfather");
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

    #[tracing::instrument(parent = &ctx.parent_span, skip_all)]
    async fn inner_normal(self, ctx: Context) {
        tracing::info!("[NORMAL] Inner Greeter");
    }
}

impl ShortGreeter {
    #[tracing::instrument(skip_all)]
    async fn short_greet(&mut self) -> ActorResult<()> {
        self.number_of_greets += 1;

        let ctx = Context::default();
        let inner_adr = self.inner.clone();
        call!(self.inner.inner_actor(ctx.clone(), inner_adr));

        let inner = InnerGreeterNormal::new();
        inner.inner_normal(Context::default()).await;

        info!(
            "Short Greeter: Number {}, Since Start: {}ms",
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
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap()
        .add_directive("gst_utils=debug".parse().unwrap())
        .add_directive("timeline=info".parse().unwrap())
        .add_directive("state=info".parse().unwrap())
        .add_directive("opset=warn".parse().unwrap());

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
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
                call!(short_greeter.short_greet());
            }
        })
    };

    let short_greeter = short_greeter.clone();
    send!(short_greeter.short_greet());

    short_greeter.termination().await
}
