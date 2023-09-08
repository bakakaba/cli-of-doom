use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    error,
    options::{ClientOptions, FindOptions},
    Client,
};

async fn initialize_client() -> Result<Client, error::Error> {
    let connection_string = std::env::var("ZENU_MONGO_URL").unwrap();
    let mut client_options = ClientOptions::parse(connection_string).await?;
    client_options.app_name = Some("Zenu".to_string());

    Client::with_options(client_options)
}

pub async fn list_databases() -> Result<(), error::Error> {
    let client = initialize_client().await.unwrap();
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    Ok(())
}

pub async fn truncate_outbound_requests_results() -> Result<(), error::Error> {
    let client = initialize_client().await.unwrap();
    let db = client.database("zenu-db");
    let col = db.collection::<Document>("outboundRequests");
    let find_options = FindOptions::builder().build();
    let mut cursor = col.find(doc! {}, find_options).await?;

    let mut count = 0;
    while let Some(next) = cursor.next().await {
        let doc = next?;
        let id = doc.get("_id").unwrap().as_str().unwrap();
        let body = doc.get("responseBody").expect("Missing response body");
        let mut json = serde_json::to_string(&body).expect("Unable to serialize");
        if json.len() <= 2048 {
            continue;
        }

        json.truncate(2048);
        col.update_one(
            doc! { "_id": id },
            doc! { "$set": {"responseBody": json}},
            None,
        )
        .await
        .unwrap();

        count += 1;
        if count % 1000 == 0 {
            println!("Processed {} documents", count);
        }
    }

    Ok(())
}
