use server::{get_auth_token, get_lyrics};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let _ = get_auth_token().await;
    let _ = get_lyrics("song/10225840?text_format=plain").await;

    Ok(())
}
