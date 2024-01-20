/// # Database logic
/// The general flow of the database is as follows:
/// 1. In order to connect to the database check if one already exists
/// 2. If it does exist, connect to it
/// 3. If it does not exist, create it and then connect to it
/// 4. Before using the database, run migrations
/// 5. Allow using the database
/// This logic is expresed via structs e.g. it is not possbile to connect to a database without
/// running migrations first.
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite:sqlite.db";

pub struct Database {
    connection: SqlitePool,
}

pub struct ConnectedDatabase {
    connection: SqlitePool,
}

impl Database {
    pub async fn connect() -> ConnectedDatabase {
        match Sqlite::database_exists(DB_URL).await {
            // Exists so just connect
            Ok(true) => match SqlitePool::connect(DB_URL).await {
                Ok(connection) => ConnectedDatabase { connection },
                Err(error) => panic!("Failed to connect to existing DB: {}", error),
            },
            // Does not exist, so create and connect
            Ok(false) => match Sqlite::create_database(DB_URL).await {
                Ok(_) => match SqlitePool::connect(DB_URL).await {
                    Ok(connection) => ConnectedDatabase { connection },
                    Err(error) => panic!("Created DB, but failed to connect: {}", error),
                },
                Err(error) => panic!("Error creating DB: {}", error),
            },
            // Sqlx error when checking if db exists
            Err(error) => panic!("Error checking if database exists: {}", error),
        }
    }

    pub fn connection(self) -> SqlitePool {
        self.connection
    }
}

impl ConnectedDatabase {
    pub async fn migrate(self) -> Database {
        match sqlx::migrate!().run(&self.connection).await {
            Ok(_) => Database {
                connection: self.connection,
            },
            Err(error) => panic!("Error running migrations: {}", error),
        }
    }
}
