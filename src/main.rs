mod simple_app;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    simple_app::run().await?;
    Ok(())
}