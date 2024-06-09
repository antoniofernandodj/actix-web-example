use std::env;
use sea_orm::{Database, DatabaseConnection, DbErr};


pub async fn init() -> DatabaseConnection {
    let database_url: String = env::var("DATABASE_URL").unwrap();

    let db: Result<DatabaseConnection, DbErr> = Database::connect(&database_url).await;
    
    if let Ok(connection) = db {
        connection
    } else {
        panic!("Cannot connect to database!")
    }

}
