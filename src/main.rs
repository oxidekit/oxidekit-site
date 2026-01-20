//! OxideKit Marketing Website
//!
//! Built with OxideKit - dogfooding the platform.

use oxide_runtime::Application;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let app = Application::from_manifest("oxide.toml")?;
    app.run()
}
