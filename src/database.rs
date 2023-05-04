use user;
extern crate mongodb;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

pub mod database {
    #[tokio::main]
    async fn connect() -> mongodb::error::Result<()> {
        let mut client_options =
            ClientOptions::parse("mongodb+srv://patrykstruzek17:<password>@cluster1.r40ouml.mongodb.net/?retryWrites=true&w=majority")
            .await?;
        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        // Get a handle to the cluster
        let client = Client::with_options(client_options)?;
        // Ping the server to see if you can connect to the cluster
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");
        Ok(())
    }
}