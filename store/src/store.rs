use diesel::prelude::*;
use crate::config::Config;
use diesel::ConnectionError;
pub struct 
Store{
    pub conn: PgConnection, // This would be your database connection type
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let db_url = config.db_url;
        // Establish a connection to the database using the URL from the config
        // This assumes you have diesel set up with the appropriate database driver
        // and that the DATABASE_URL environment variable is set correctly.
        // If the connection fails, it will return an error.
        // If it succeeds, it will return an instance of Store with the connection.
        // Note: You might want to handle the error more gracefully in a real application.
        // For now, we will just panic if the connection fails.
        // You can replace this with proper error handling as needed.
        // This is a placeholder for the actual connection logic.
        // Ensure you have the diesel crate and the appropriate database driver in your Cargo.toml
        // file.
        // Example: diesel = { version = "1.4", features = ["postgres"] }
        // Ensure you have the diesel crate and the appropriate database driver in your Cargo.toml
        // file.
        let conn = PgConnection::establish(&db_url)?;
        Ok(Self {
            conn
        })
    }
}