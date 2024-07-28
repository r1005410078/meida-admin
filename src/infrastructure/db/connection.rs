use diesel::{r2d2::ConnectionManager, MysqlConnection};

pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection(db_url: &str) -> DBPool {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
