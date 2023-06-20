use server::{connect, get_auth_token};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = connect().await;
    let _ = get_auth_token().await;

    Ok(())
}
