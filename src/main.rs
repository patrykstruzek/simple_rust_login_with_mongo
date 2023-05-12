mod app;
mod database;
mod input;
mod user;

use crate::app::run;


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    run()?;
    Ok(())
}