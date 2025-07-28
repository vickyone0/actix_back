// use diesel::PgConnection;
// use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, Error as R2D2Error};
// use dotenv::dotenv;
// use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

// pub fn establish_connection() -> DbPool {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     Pool::builder()
//         .build(manager)
//         .expect("Failed to create pool.")
// }
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}