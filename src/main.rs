use make87::config::load_config_from_default_env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = load_config_from_default_env()?;

    make87::run_forever();
    Ok(())
}
