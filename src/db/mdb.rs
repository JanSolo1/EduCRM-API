use mongodb::{ Client, Collection };
use crate::db::helpers::Student;
use dotenv::var;

pub async fn get_collection(db_name: &str, collection_name: &str) -> Collection<Student> {
    dotenv::dotenv().ok();
    let uri = var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri).await.unwrap();
    client.database(db_name).collection(collection_name)
}
