use diesel::mysql::MysqlConnection;
use diesel::Connection;

pub fn establish_connection(database_url: &str) -> MysqlConnection {
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

