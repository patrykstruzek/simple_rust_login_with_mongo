pub mod database {
    use mongodb::{
        bson,
        bson::doc,
        options::{ClientOptions, ServerApi, ServerApiVersion},
        Client,
    };
    use crate::user::User;

    #[tokio::main]
    pub async fn connect() -> mongodb::error::Result<Client> {
        let mut client_options =
            ClientOptions::parse("mongodb+srv://patrykstruzek17:<password>@cluster1.r40ouml.mongodb.net/?retryWrites=true&w=majority")?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options)?;
        client.database("Users").run_command(doc! {"ping": 1}, None).await?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");
        Ok(client)
    }
    #[tokio::main]
    pub async fn insert_user(client: &Client, user: User) -> mongodb::error::Result<()> {
        let coll = client.database("Users").collection("users");

        let user_bson = bson::to_bson(&user).unwrap();

        if let bson::Bson::Document(document) = user_bson {
            coll.insert_one(document, None).await?;
        }
        println!("Successfully added user!");
        Ok(())
    }
}
