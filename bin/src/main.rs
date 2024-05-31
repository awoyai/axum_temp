use config::CFG;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use utils::env::{self, RT};
use axum::Router;

fn main() {
    RT.block_on(async {
        let log_env = env::get_log_level();
        let format = env::get_log_format();
        // 文件输出
        let file_append = tracing_appender::rolling::daily(&CFG.log.dir, &CFG.log.file);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_append);
        // 控制台输出
        let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
        let logger = Registry::default()
            .with(EnvFilter::from_default_env().add_directive(log_env.into()))
            .with(fmt::Layer::default().with_writer(std_non_blocking).event_format(format.clone()).pretty())
            .with(fmt::Layer::default().with_writer(non_blocking).event_format(format))
            // .with(console_layer)
            ;
        tracing::subscriber::set_global_default(logger).unwrap();
        let app = Router::new().nest(&CFG.server.api_prefix, api::api());
        let listener = tokio::net::TcpListener::bind(&CFG.server.address).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    })
}
