use std::env;
use tracing::Level;
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Sets `RUST_BACKTRACE=1` as default so we always get a full stacktrace
/// on an error.
pub(crate) fn set_full_stacktrace_as_default() {
    let key = "RUST_BACKTRACE";

    let is_value_already_set = env::var(key).is_ok();
    if is_value_already_set {
        return;
    }
    env::set_var(key, "1");
}

/// Enable logging with configurable verbosity.
/// If RUST_LOG is set, it will be used; otherwise, default levels are applied.
pub(crate) fn enable_logging(verbose: bool) {
    let key = "RUST_LOG";
    let value = match env::var(key) {
        Ok(val) => val,
        Err(_) => String::new(),
    };

    if !value.is_empty() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
        tracing::info!("Use RUST_LOG={} env variable to set logging output.", value);
        if verbose {
            tracing::warn!("RUST_LOG= environment variable is already set. Ignore --verbose flag.")
        }
        return;
    }

    let regular_filter = tracing_subscriber::filter::Targets::new()
        .with_target("pkdns", Level::INFO)
        .with_target("mainline", Level::WARN);

    let verbose_filter = tracing_subscriber::filter::Targets::new()
        .with_target("pkdns", Level::DEBUG)
        .with_target("mainline", Level::WARN);

    let mut filter: Targets = regular_filter;
    if verbose {
        filter = verbose_filter;
    }

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    if verbose {
        tracing::info!("Verbose mode enabled.");
    }
}

/// Enable spaces-specific logging with configurable verbosity.
/// Note: This function assumes the global tracing subscriber is already initialized.
/// If SPACES_LOG is set, it will be used; otherwise, default levels are applied.
pub(crate) fn enable_spaces_logging(verbose: bool) {
    let key = "SPACES_LOG";
    let value = match env::var(key) {
        Ok(val) => val,
        Err(_) => String::new(),
    };

    if !value.is_empty() {
        tracing::info!("Used SPACES_LOG={} env variable to set logging output.", value);
        if verbose {
            tracing::warn!("SPACES_LOG= is set. Ignore --verbose flag.");
        }
        return;
    }

    if verbose {
        tracing::info!("Spaces Verbose mode enabled.");
    }
}

/// Wait until the user hits CTRL+C
pub(crate) async fn wait_on_ctrl_c() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal Ctrl+C: {}", err);
        }
    }
}
