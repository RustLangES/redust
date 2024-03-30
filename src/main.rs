use redust::Redust;

mod client_state;
mod commands;
mod memory;
mod redust;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Redust::new("127.0.0.1:6969").await?.run().await;
    return Ok(());
}
