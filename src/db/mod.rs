use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::time::Duration;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn establish_connection() -> Pool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

   r2d2::Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .connection_timeout(Duration::from_secs(30))
        .idle_timeout(Some(Duration::from_secs(600)) )
        .build(manager)
        .expect("Failed to create pool.")
}