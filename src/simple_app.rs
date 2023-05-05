pub mod simple_app {
    mod database;
    mod user;

    #[tokio::main]
    pub async fn run() -> mongodb::error::Result<()> {
        database::connect();
        database::insert_user(user::User::create_user()).await?.unwrap();
        Ok(())
    }
}