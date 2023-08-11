use mongodb::{error, options::ClientOptions, Client};

async fn initialize_client() -> Result<Client, error::Error> {
    let connection_string = std::env::var("ZENU_MONGO_URL").unwrap();
    let mut client_options = ClientOptions::parse(connection_string).await?;
    client_options.app_name = Some("Zenu".to_string());

    Client::with_options(client_options)
}

// let client = initialize_client();

pub async fn list_databases() -> Result<(), error::Error> {
    let client = initialize_client().await.unwrap();
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(())
}
