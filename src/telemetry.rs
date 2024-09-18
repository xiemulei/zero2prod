use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;

/// 将多个层次组合成 `tracing` 的订阅器
///
/// # 注意事项
///
/// 将 `impl Subscriber` 作为返回值的类型，以避免写出繁琐的真实类型
/// 我们需要显式地讲返回类型标记为 `Send` 和 `Sync`，以便后面可以将其传递给 `init_subscriber`
pub fn get_subscriber<Sink>(name: String, env_filter: String, sink: Sink) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // 将格式化的跨度输出到 stdout
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// 将一个订阅器设置为全局默认值，用于处理所有跨度数据
///
/// 这个函数只可被调用一次
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    // 设置 `tracing` 的默认订阅器
    set_global_default(subscriber).expect("Failed to set a global subscriber");
}