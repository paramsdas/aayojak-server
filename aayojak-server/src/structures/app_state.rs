use std::sync::Mutex;

use diesel::PgConnection;

pub struct AppState {
    pub pg_connection: Mutex<PgConnection>,
}
