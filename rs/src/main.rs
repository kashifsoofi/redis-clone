mod parse_error;
mod parser;
mod resp;
mod server;

use anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::start("127.0.0.1:6379".to_string()).await?;
    Ok(())
}
