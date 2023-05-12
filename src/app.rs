use crate::database;
use crate::user::User;


pub async fn run() -> Result<(), mongodb::error::Error> {
    let client = database::connect().await?;
    database::insert_user(&client, User::create_user()).await?;
    Ok(())
}
