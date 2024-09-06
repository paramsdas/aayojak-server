use diesel::{Connection, PgConnection};

pub fn establish_postgres_connection(pg_url: &str) -> PgConnection {
    PgConnection::establish(pg_url)
        .unwrap_or_else(|_| panic!("Error connecting to the specified url"))
}
