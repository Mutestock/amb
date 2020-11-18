use dotenv::dotenv;
use std::env;

// https://github.com/sfackler/r2d2https://github.com/sfackler/r2d2
// https://docs.diesel.rs/diesel/r2d2/struct.ConnectionManager.html
// https://docs.diesel.rs/diesel/pg/struct.PgConnection.html
use diesel::r2d2::ConnectionManager;
use diesel::{
    PgConnection,
    //Testing
    Connection
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    pub static ref POOL: Pool = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    };
}

// Manual connection without pooling. The basic method. Only used for testing via a route.
pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.");
    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}