use redust::Redust;

mod client_state;
mod commands;
mod config;
mod memory;
mod redust;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Redust::new().await?.run().await;
    return Ok(());
}
