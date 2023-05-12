use crate::database;
use crate::user::User;

#[tokio::main]
pub async fn run() -> mongodb::error::Result<()> {
    let client = database::connect()?;
    database::insert_user(&client, User::create_user())?;
    Ok(())
}
